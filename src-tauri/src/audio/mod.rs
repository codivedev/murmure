pub mod recorder;
pub mod encoder;

pub use recorder::AudioRecorder;
pub use encoder::encode_to_opus;

use crate::error::{AppError, Result};
use crate::types::AudioConfig;

pub fn record_and_encode(recorder: &mut AudioRecorder, config: &AudioConfig) -> Result<Vec<u8>> {
    let pcm_samples = recorder.stop_recording()?;
    let opus_data = encode_to_opus(&pcm_samples, config)?;
    Ok(opus_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SampleFormat;
    
    #[test]
    fn test_record_and_encode_function_signature() {
        let config = AudioConfig {
            sample_rate: 16000,
            channels: 1,
            sample_format: SampleFormat::F32,
        };
        
        let mut recorder = AudioRecorder::new(config.clone()).unwrap();
    }
}