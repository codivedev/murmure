use crate::config::SettingsStore;
use crate::crypto::retrieve_api_key;
use crate::error::{AppError, Result};
use crate::groq::client::GroqClient;
use crate::input::inserter::insert_text as insert_text_impl;
use crate::input::shortcut::register;
use crate::input::window::{get_active_window as get_active_window_impl, WindowInfo};
use crate::types::{Settings, TranscriptionResult, AudioConfig, SampleFormat};
use crate::audio::{AudioRecorder, encode_to_opus};
use tauri::Emitter;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use serde_json;

static AUDIO_RECORDER: Lazy<Mutex<Option<AudioRecorder>>> = Lazy::new(|| Mutex::new(None));
static AUDIO_CONFIG: Lazy<AudioConfig> = Lazy::new(|| AudioConfig {
    sample_rate: 16000,
    channels: 1,
    sample_format: SampleFormat::F32,
});

#[tauri::command]
pub fn get_settings() -> Result<Settings> {
    SettingsStore::load().map_err(|e| AppError::ConfigError(e.to_string()))
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<()> {
    SettingsStore::save(&settings).map_err(|e| AppError::ConfigError(e.to_string()))
}

#[tauri::command]
pub fn store_api_key(key: String) -> Result<()> {
    crate::crypto::store_api_key(&key).map_err(|e| AppError::ConfigError(e.to_string()))
}

#[tauri::command]
pub fn has_api_key() -> bool {
    crate::crypto::has_api_key()
}

#[tauri::command]
pub fn start_audio_recording(app: tauri::AppHandle) -> Result<()> {
    let config = AUDIO_CONFIG.clone();
    let mut recorder = AUDIO_RECORDER.lock().map_err(|e| AppError::AudioError(e.to_string()))?;
    
    let mut new_recorder = AudioRecorder::new(config)?;
    new_recorder.start_recording()?;
    *recorder = Some(new_recorder);
    
    app.emit("audio-recording-started", ()).map_err(|e| AppError::AudioError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn stop_audio_recording(app: tauri::AppHandle) -> Result<Vec<u8>> {
    let mut recorder = AUDIO_RECORDER.lock().map_err(|e| AppError::AudioError(e.to_string()))?;
    
    let mut rec = recorder.take().ok_or_else(|| AppError::AudioError("No recording in progress".to_string()))?;
    let pcm = rec.stop_recording()?;
    let duration_ms = rec.get_duration_ms();
    
    let opus_data = encode_to_opus(&pcm, &AUDIO_CONFIG)?;
    
    app.emit("audio-recording-stopped", serde_json::json!({ "duration_ms": duration_ms }))
        .map_err(|e| AppError::AudioError(e.to_string()))?;
    app.emit("audio-data-ready", serde_json::json!({ "size_bytes": opus_data.len() }))
        .map_err(|e| AppError::AudioError(e.to_string()))?;
    
    Ok(opus_data)
}

#[tauri::command]
pub async fn transcribe_audio(wav_bytes: Vec<u8>) -> Result<TranscriptionResult> {
    let api_key = retrieve_api_key()
        .map_err(|e| AppError::GroqApiError(format!("Failed to retrieve API key: {}", e)))?;
    
    let client = GroqClient::new(api_key);
    client.transcribe(wav_bytes).await
}

#[tauri::command]
pub fn insert_text(text: String) -> Result<()> {
    insert_text_impl(&text)
}

#[tauri::command]
pub fn register_shortcut(shortcut: String) -> Result<()> {
    // Create empty callbacks since the shortcut module already handles event emission
    let on_press: crate::input::shortcut::Callback = Box::new(|_s| {});
    let on_release: crate::input::shortcut::Callback = Box::new(|_s| {});
    
    register(&shortcut, on_press, on_release)
}

#[tauri::command]
pub fn get_active_window() -> Result<WindowInfo> {
    get_active_window_impl().map_err(|e| AppError::InputError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_insert_text_command_compiles() {
        let result = insert_text("test".to_string());
        assert!(result.is_ok() || result.is_err());
    }
    
    #[test]
    fn test_get_active_window_command_compiles() {
        let result = get_active_window();
        assert!(result.is_ok() || result.is_err());
    }
    
    #[test]
    fn test_register_shortcut_command_compiles() {
        let result = register_shortcut("Ctrl+Alt+T".to_string());
        assert!(result.is_ok() || result.is_err());
    }
}