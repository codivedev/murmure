use crate::error::{AppError, Result};
use crate::types::AudioConfig;
use opus::{Application, Encoder, Channels, Bitrate};
use ogg::writing::{PacketWriter, PacketWriteEndInfo};
use std::io::Cursor;

const MAX_AUDIO_SIZE_BYTES: usize = 25 * 1024 * 1024;

fn create_opus_header(channels: u8, sample_rate: u32) -> Vec<u8> {
    let mut header = Vec::new();
    header.extend_from_slice(b"OpusHead");
    header.push(1);
    header.push(channels);
    header.extend_from_slice(&0u16.to_le_bytes());
    header.extend_from_slice(&sample_rate.to_le_bytes());
    header.extend_from_slice(&0i16.to_le_bytes());
    header.push(0);
    header
}

pub fn encode_to_opus(pcm: &[f32], config: &AudioConfig) -> Result<Vec<u8>> {
    if config.channels != 1 {
        return Err(AppError::AudioError("Only mono audio is supported for Opus encoding".to_string()));
    }
    
    let channels = Channels::Mono;
    let sample_rate = config.sample_rate;
    let application = Application::Audio;
    
    let mut encoder = Encoder::new(sample_rate, channels, application)
        .map_err(|e| AppError::AudioError(format!("Failed to create Opus encoder: {:?}", e)))?;
    
    encoder.set_bitrate(Bitrate::Bits(24000))
        .map_err(|e| AppError::AudioError(format!("Failed to set Opus bitrate: {:?}", e)))?;
    
    let mut cursor = Cursor::new(Vec::new());
    let mut packet_writer = PacketWriter::new(&mut cursor);
    
    let header_packet = create_opus_header(config.channels as u8, config.sample_rate);
    let serial = 12345u32;
    packet_writer.write_packet(header_packet, serial, PacketWriteEndInfo::EndPage, 0)
        .map_err(|e| AppError::AudioError(format!("Failed to write Opus header to OGG: {}", e)))?;
    
    const FRAME_SIZE: usize = 960;
    
    let mut packet_count = 1;
    let mut remaining_samples = pcm;
    
    while !remaining_samples.is_empty() {
        let frame_samples_vec: Vec<f32> = if remaining_samples.len() >= FRAME_SIZE {
            remaining_samples[..FRAME_SIZE].to_vec()
        } else {
            let mut padded_frame = vec![0.0f32; FRAME_SIZE];
            padded_frame[..remaining_samples.len()].copy_from_slice(remaining_samples);
            remaining_samples = &[];
            padded_frame
        };
        
        let encoded_packet = encoder.encode_vec_float(&frame_samples_vec, 4000)
            .map_err(|e| AppError::AudioError(format!("Failed to encode Opus frame: {:?}", e)))?;
        
        let is_last_packet = remaining_samples.is_empty();
        let end_info = if is_last_packet {
            PacketWriteEndInfo::EndStream
        } else {
            PacketWriteEndInfo::NormalPacket
        };
        packet_writer.write_packet(encoded_packet, serial, end_info, packet_count as u64)
            .map_err(|e| AppError::AudioError(format!("Failed to write Opus packet to OGG: {}", e)))?;
        
        packet_count += 1;
        
        if remaining_samples.len() >= FRAME_SIZE {
            remaining_samples = &remaining_samples[FRAME_SIZE..];
        }
    }
    

    
    let ogg_data = cursor.into_inner();
    
    if ogg_data.len() > MAX_AUDIO_SIZE_BYTES {
        return Err(AppError::AudioError(
            format!(
                "Encoded audio size {} bytes exceeds Groq API limit of {} bytes",
                ogg_data.len(),
                MAX_AUDIO_SIZE_BYTES
            )
        ));
    }
    
    Ok(ogg_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AudioConfig, SampleFormat};
    
    #[test]
    fn test_encode_opus_mono() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        
        let mut pcm = Vec::new();
        for i in 0..16000 {
            let t = i as f32 / 16000.0;
            let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin();
            pcm.push(sample);
        }
        
        let opus_data = encode_to_opus(&pcm, &config).unwrap();
        assert!(!opus_data.is_empty());
        assert!(opus_data.len() <= MAX_AUDIO_SIZE_BYTES);
        assert_eq!(&opus_data[0..4], b"OggS");
        assert!(opus_data.windows(8).any(|window| window == b"OpusHead"));
    }
    
    #[test]
    fn test_encode_stereo_not_supported() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 2,
            sample_format: SampleFormat::F32,
        };
        
        let mut pcm = Vec::new();
        for i in 0..16000 {
            let t = i as f32 / 16000.0;
            let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin();
            pcm.push(sample);
            pcm.push(sample);
        }
        
        let result = encode_to_opus(&pcm, &config);
        assert!(result.is_err());
        if let Err(AppError::AudioError(msg)) = result {
            assert!(msg.contains("Only mono audio is supported"));
        } else {
            panic!("Expected AudioError");
        }
    }
    
    #[test]
    fn test_size_limit_exceeded() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        
        let huge_pcm = vec![0.5f32; 16000 * 10000];
        
        let result = encode_to_opus(&huge_pcm, &config);
        assert!(result.is_err());
        if let Err(AppError::AudioError(msg)) = result {
            assert!(msg.contains("exceeds Groq API limit"));
        } else {
            panic!("Expected AudioError");
        }
    }
}