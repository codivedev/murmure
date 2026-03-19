use serde::{Deserialize, Serialize};

/// Audio configuration - Used by Plan 2 (Audio)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,  // 16000
    pub channels: u16,     // 1 (mono)
    pub sample_format: SampleFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SampleFormat {
    F32,
    I16,
}

/// Transcription result - Used by Plan 3 (Groq)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub duration_ms: u64,
    pub language: Option<String>,
}

/// App state - Used by Plan 4 (Frontend)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppState {
    Idle,
    Recording { duration_ms: u64 },
    Processing,
    Success { text: String },
    Error { message: String },
}

/// Settings - Used by all plans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub shortcut: String,
    pub language: String,
    pub overlay_position: OverlayPosition,
    pub setup_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayPosition {
    pub x: i32,
    pub y: i32,
}