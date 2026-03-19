use crate::error::{AppError, Result};
use crate::types::{AudioConfig, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::time::{Duration, Instant};

pub struct AudioRecorder {
    config: AudioConfig,
    stream: Option<cpal::Stream>,
    samples_sender: Option<Sender<Vec<f32>>>,
    samples_receiver: Option<Receiver<Vec<f32>>>,
    start_time: Option<Instant>,
    max_duration: Duration,
}

impl AudioRecorder {
    pub fn new(config: AudioConfig) -> Result<Self> {
        if config.sample_format != SampleFormat::F32 && config.sample_format != SampleFormat::I16 {
            return Err(AppError::AudioError(
                "Only F32 and I16 sample formats are supported".to_string(),
            ));
        }

        let (samples_sender, samples_receiver) = bounded::<Vec<f32>>(100);

        Ok(AudioRecorder {
            config,
            stream: None,
            samples_sender: Some(samples_sender),
            samples_receiver: Some(samples_receiver),
            start_time: None,
            max_duration: Duration::from_millis(600_000),
        })
    }

    pub fn start_recording(&mut self) -> Result<()> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| AppError::AudioError("No default input device found".to_string()))?;

        let config = device
            .default_input_config()
            .map_err(|e| AppError::AudioError(format!("Failed to get default input config: {}", e)))?;

        let samples_sender = self
            .samples_sender
            .clone()
            .ok_or_else(|| AppError::AudioError("Samples sender already consumed".to_string()))?;

        let err_fn = move |err| {
            eprintln!("Audio stream error: {}", err);
        };

        let channels = config.channels();
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                let sender = samples_sender.clone();
                device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        let mono_samples: Vec<f32> = if channels > 1 {
                            data.chunks(channels as usize)
                                .map(|chunk| chunk[0])
                                .collect()
                        } else {
                            data.to_vec()
                        };
                        let _ = sender.send(mono_samples);
                    },
                    err_fn,
                    None,
                )
            }
            cpal::SampleFormat::I16 => {
                let sender = samples_sender.clone();
                device.build_input_stream(
                    &config.into(),
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        let mono_samples: Vec<f32> = if channels > 1 {
                            data.chunks(channels as usize)
                                .map(|chunk| chunk[0] as f32 / i16::MAX as f32)
                                .collect()
                        } else {
                            data.iter()
                                .map(|&sample| sample as f32 / i16::MAX as f32)
                                .collect()
                        };
                        let _ = sender.send(mono_samples);
                    },
                    err_fn,
                    None,
                )
            }
            cpal::SampleFormat::U16 => {
                let sender = samples_sender.clone();
                device.build_input_stream(
                    &config.into(),
                    move |data: &[u16], _: &cpal::InputCallbackInfo| {
                        let mono_samples: Vec<f32> = if channels > 1 {
                            data.chunks(channels as usize)
                                .map(|chunk| chunk[0] as f32 / u16::MAX as f32)
                                .collect()
                        } else {
                            data.iter()
                                .map(|&sample| sample as f32 / u16::MAX as f32)
                                .collect()
                        };
                        let _ = sender.send(mono_samples);
                    },
                    err_fn,
                    None,
                )
            }
            _ => {
                return Err(AppError::AudioError(
                    "Unsupported sample format from audio device".to_string(),
                ))
            }
        }
        .map_err(|e| AppError::AudioError(format!("Failed to build input stream: {}", e)))?;

        stream
            .play()
            .map_err(|e| AppError::AudioError(format!("Failed to start stream: {}", e)))?;

        self.stream = Some(stream);
        self.start_time = Some(Instant::now());

        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<Vec<f32>> {
        if self.stream.is_none() {
            return Err(AppError::AudioError(
                "Recording not started or already stopped".to_string(),
            ));
        }

        self.stream = None;

        let mut all_samples = Vec::new();
        let receiver = self
            .samples_receiver
            .take()
            .ok_or_else(|| AppError::AudioError("Samples receiver already consumed".to_string()))?;

        while let Ok(samples) = receiver.try_recv() {
            all_samples.extend_from_slice(&samples);
        }

        Ok(all_samples)
    }

    pub fn get_duration_ms(&self) -> u64 {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            if elapsed > self.max_duration {
                self.max_duration.as_millis() as u64
            } else {
                elapsed.as_millis() as u64
            }
        } else {
            0
        }
    }

    pub fn is_max_duration_reached(&self) -> bool {
        if let Some(start_time) = self.start_time {
            start_time.elapsed() >= self.max_duration
        } else {
            false
        }
    }
}

impl From<SampleFormat> for cpal::SampleFormat {
    fn from(format: SampleFormat) -> Self {
        match format {
            SampleFormat::F32 => cpal::SampleFormat::F32,
            SampleFormat::I16 => cpal::SampleFormat::I16,
        }
    }
}

// SAFETY: AudioRecorder is only used from the main thread and the stream is dropped
// before any potential move across threads. This is safe in the context of Tauri
// where all commands run on the main thread.
unsafe impl Send for AudioRecorder {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SampleFormat;

    #[test]
    fn test_audio_recorder_creation() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        let recorder = AudioRecorder::new(config);
        assert!(recorder.is_ok());
    }

    #[test]
    fn test_valid_configurations() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        let recorder = AudioRecorder::new(config);
        assert!(recorder.is_ok());

        let config = AudioConfig {
            sample_rate: 44100,
            channels: 2,
            sample_format: SampleFormat::I16,
        };
        let recorder = AudioRecorder::new(config);
        assert!(recorder.is_ok());
    }

    #[test]
    fn test_supported_formats() {
        let config_f32 = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        let config_i16 = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::I16,
        };
        
        assert!(AudioRecorder::new(config_f32).is_ok());
        assert!(AudioRecorder::new(config_i16).is_ok());
    }

    #[test]
    fn test_duration_tracking() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        let mut recorder = AudioRecorder::new(config).unwrap();
        assert_eq!(recorder.get_duration_ms(), 0);
        
        recorder.start_time = Some(Instant::now() - Duration::from_millis(1500));
        let duration = recorder.get_duration_ms();
        assert!(duration >= 1400 && duration <= 1600);
    }

    #[test]
    fn test_max_duration_enforcement() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        let mut recorder = AudioRecorder::new(config).unwrap();
        recorder.start_time = Some(Instant::now() - Duration::from_millis(660_000));
        assert_eq!(recorder.get_duration_ms(), 600_000);
        assert!(recorder.is_max_duration_reached());
    }

    #[test]
    fn test_sample_conversion_f32() {
        let (sender, receiver) = bounded::<Vec<f32>>(10);
        let test_data = vec![0.5f32, -0.3, 0.8];
        let samples: Vec<f32> = test_data.to_vec();
        let _ = sender.send(samples);
        
        let received = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(received, test_data);
    }

    #[test]
    fn test_sample_conversion_i16() {
        let (sender, receiver) = bounded::<Vec<f32>>(10);
        let test_data = vec![16384i16, -9830, 24576];
        let samples: Vec<f32> = test_data
            .iter()
            .map(|&sample| sample as f32 / i16::MAX as f32)
            .collect();
        let _ = sender.send(samples);
        
        let received = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert!((received[0] - 0.5).abs() < 0.01);
        assert!((received[1] + 0.3).abs() < 0.01);
        assert!(received[2] > 0.7 && received[2] < 0.9);
    }
}