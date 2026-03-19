use crate::error::{AppError, Result};
use crate::types::AudioConfig;

const MAX_AUDIO_SIZE_BYTES: usize = 25 * 1024 * 1024;

pub fn encode_to_wav(pcm: &[f32], config: &AudioConfig) -> Result<Vec<u8>> {
    let (bits_per_sample, bytes_per_sample, audio_format) = match config.sample_format {
        crate::types::SampleFormat::F32 => (32, 4, 3u16),
        crate::types::SampleFormat::I16 => (16, 2, 1u16),
    };
    
    let num_samples = pcm.len();
    let data_size = (num_samples * bytes_per_sample) as u32;
    let file_size = 36 + data_size;
    
    let mut wav_data = Vec::with_capacity((file_size + 8) as usize);
    
    wav_data.extend_from_slice(b"RIFF");
    wav_data.extend_from_slice(&file_size.to_le_bytes());
    wav_data.extend_from_slice(b"WAVE");
    
    wav_data.extend_from_slice(b"fmt ");
    wav_data.extend_from_slice(&16u32.to_le_bytes());
    wav_data.extend_from_slice(&audio_format.to_le_bytes());
    wav_data.extend_from_slice(&(config.channels as u16).to_le_bytes());
    wav_data.extend_from_slice(&config.sample_rate.to_le_bytes());
    
    let byte_rate = config.sample_rate * config.channels as u32 * bytes_per_sample as u32;
    wav_data.extend_from_slice(&byte_rate.to_le_bytes());
    
    let block_align = (config.channels * bytes_per_sample as u16) as u16;
    wav_data.extend_from_slice(&block_align.to_le_bytes());
    wav_data.extend_from_slice(&(bits_per_sample as u16).to_le_bytes());
    
    wav_data.extend_from_slice(b"data");
    wav_data.extend_from_slice(&data_size.to_le_bytes());
    
    match config.sample_format {
        crate::types::SampleFormat::F32 => {
            for &sample in pcm {
                let clamped = sample.clamp(-1.0, 1.0);
                let bytes = clamped.to_le_bytes();
                wav_data.extend_from_slice(&bytes);
            }
        }
        crate::types::SampleFormat::I16 => {
            for &sample in pcm {
                let clamped = sample.clamp(-1.0, 1.0);
                let i16_sample = (clamped * i16::MAX as f32) as i16;
                wav_data.extend_from_slice(&i16_sample.to_le_bytes());
            }
        }
    }
    
    if wav_data.len() > MAX_AUDIO_SIZE_BYTES {
        return Err(AppError::AudioError(
            format!(
                "Encoded audio size {} bytes exceeds Groq API limit of {} bytes",
                wav_data.len(),
                MAX_AUDIO_SIZE_BYTES
            )
        ));
    }
    
    Ok(wav_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AudioConfig, SampleFormat};
    
    #[test]
    fn test_encode_f32_mono() {
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
        
        let wav_data = encode_to_wav(&pcm, &config).unwrap();
        assert!(!wav_data.is_empty());
        assert!(wav_data.len() <= MAX_AUDIO_SIZE_BYTES);
        assert_eq!(&wav_data[0..4], b"RIFF");
        assert_eq!(&wav_data[8..12], b"WAVE");
    }
    
    #[test]
    fn test_encode_i16_mono() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::I16,
        };
        
        let mut pcm = Vec::new();
        for i in 0..16000 {
            let t = i as f32 / 16000.0;
            let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin();
            pcm.push(sample);
        }
        
        let wav_data = encode_to_wav(&pcm, &config).unwrap();
        assert!(!wav_data.is_empty());
        assert!(wav_data.len() <= MAX_AUDIO_SIZE_BYTES);
        assert_eq!(&wav_data[0..4], b"RIFF");
        assert_eq!(&wav_data[8..12], b"WAVE");
    }
    
    #[test]
    fn test_encode_stereo() {
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
        
        let wav_data = encode_to_wav(&pcm, &config).unwrap();
        assert!(!wav_data.is_empty());
        assert!(wav_data.len() <= MAX_AUDIO_SIZE_BYTES);
    }
    
    #[test]
    fn test_size_limit_exceeded() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        
        let huge_pcm = vec![0.5f32; 16000 * 420];
        
        let result = encode_to_wav(&huge_pcm, &config);
        assert!(result.is_err());
        if let Err(AppError::AudioError(msg)) = result {
            assert!(msg.contains("exceeds Groq API limit"));
        } else {
            panic!("Expected AudioError");
        }
    }
}