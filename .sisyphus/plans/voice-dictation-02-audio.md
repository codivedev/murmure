# Plan 2/4: Core Audio Pipeline

## TL;DR

> **Quick Summary**: Pipeline audio complet - capture microphone via cpal, encodage WAV via hound. Utilise les types du Plan 1.
>
> **Deliverables**:
> - Module audio capture (cpal) avec configuration 16kHz mono
> - Module audio encoding (hound) pour WAV
> - Recording duration tracking (max 10 min)
> - Tests unitaires
>
> **Estimated Effort**: Medium
> **Parallel Execution**: YES (with Plans 3, 4 after Plan 1)
> **Dependencies**: Plan 1 (types: AudioConfig, AppError, Result<T>)

---

## Context

### Role in Parallel Execution
Ce plan implémente le pipeline audio. Il peut démarrer **en parallèle** avec les Plans 3 et 4 une fois le Plan 1 complété.

**Dependency Graph**:
```
Plan 1 (Infrastructure) ──→ Plan 2 (Audio) ──┐
                         ──→ Plan 3 (Groq) ──┼──→ Integration
                         ──→ Plan 4 (UI) ────┘
```

### Interfaces Used (from Plan 1)

```rust
// From Plan 1 - src-tauri/src/types.rs
use crate::types::AudioConfig;
use crate::error::{AppError, Result};

// Expected interface:
pub struct AudioConfig {
    pub sample_rate: u32,  // 16000
    pub channels: u16,     // 1 (mono)
    pub sample_format: SampleFormat,
}
```

### Interfaces Provided (for Integration)

```rust
// This plan provides:
pub struct AudioRecorder {
    // Internal state
}

impl AudioRecorder {
    pub fn new(config: AudioConfig) -> Result<Self>;
    pub fn start_recording(&mut self) -> Result<()>;
    pub fn stop_recording(&mut self) -> Result<Vec<u8>>;  // Returns WAV bytes
    pub fn get_duration_ms(&self) -> u64;
}

// Events emitted via Tauri:
// - "audio-recording-started"
// - "audio-recording-stopped" { duration_ms: u64 }
// - "audio-data-ready" { size_bytes: usize }
```

---

## Work Objectives

### Core Objective
Créer un pipeline audio robuste qui capture le microphone, encode en WAV, et fournit les données pour la transcription.

### Concrete Deliverables
- `src-tauri/src/audio/recorder.rs` - Audio capture avec cpal
- `src-tauri/src/audio/encoder.rs` - WAV encoding avec hound
- `src-tauri/src/audio/mod.rs` - Module exports
- Tests unitaires pour capture et encoding

### Definition of Done
- [x] Audio capturé à 16kHz mono
- [x] WAV encoding fonctionne
- [x] Recording limité à 10 minutes
- [x] `cargo test audio` passe

---

## TODOs

- [x] 1. **Audio Capture Module (cpal)**

  **What to do**:
  - Create `src-tauri/src/audio/recorder.rs`
  - Implement `AudioRecorder` struct using `cpal` crate
  - Configure: 16kHz, mono, 16-bit PCM (or F32)
  - Functions:
    - `new(config: AudioConfig) -> Result<Self>`
    - `start_recording(&mut self) -> Result<()>`
    - `stop_recording(&mut self) -> Result<Vec<f32>>`  // Returns PCM samples
    - `get_duration_ms(&self) -> u64`
  - Use channels for real-time audio streaming
  - Handle PipeWire backend (Pop!_OS default)
  - Add 10-minute max duration with auto-stop
  - Write tests with mock audio device

  **References**:
  - `cpal` docs: `https://docs.rs/cpal/`
  - PipeWire: `https://www.pipewire.org/`

  **Acceptance Criteria**:
  - [ ] Audio captured at 16kHz mono
  - [ ] Recording stops at 10-minute limit
  - [ ] `cargo test audio::recorder` passes

  **QA Scenarios**:
  ```
  Scenario: Audio capture starts and stops
    Tool: Bash
    Preconditions: Microphone available
    Steps:
      1. Create AudioRecorder
      2. Start recording
      3. Wait 2 seconds
      4. Stop recording
      5. Verify PCM data returned
    Expected Result: Non-empty PCM data
    Evidence: .sisyphus/evidence/plan2-01-capture.log
  ```

  **Commit**: NO (groups with Task 2)

- [x] 2. **Audio Encoding Module (hound)**

  **What to do**:
  - Create `src-tauri/src/audio/encoder.rs`
  - Implement `AudioEncoder` using `hound` crate
  - Function: `encode_to_wav(pcm: &[f32], config: &AudioConfig) -> Result<Vec<u8>>`
  - Create valid WAV header
  - Calculate size to enforce 25MB Groq limit
  - Memory-efficient encoding
  - Write tests for encoding roundtrip

  **References**:
  - `hound` docs: `https://docs.rs/hound/`
  - WAV format: `http://soundfile.sapp.org/doc/WaveFormat/`

  **Acceptance Criteria**:
  - [ ] PCM encoded to valid WAV
  - [ ] WAV under 25MB for 10-minute recording
  - [ ] `cargo test audio::encoder` passes

  **QA Scenarios**:
  ```
  Scenario: WAV encoding produces valid output
    Tool: Bash
    Steps:
      1. Generate test PCM (1 second silence)
      2. Encode to WAV
      3. Verify WAV header valid
    Expected Result: Valid WAV file
    Evidence: .sisyphus/evidence/plan2-02-encode.log
  ```

  **Commit**: NO (groups with Task 2)

- [x] 3. **Audio Module Integration**

  **What to do**:
  - Create `src-tauri/src/audio/mod.rs`
  - Export `AudioRecorder`, `AudioEncoder`
  - Add convenience function: `record_and_encode(duration_ms: u64) -> Result<Vec<u8>>`
  - Add Tauri events emission:
    - `audio-recording-started`
    - `audio-recording-stopped`
    - `audio-data-ready`
  - Write integration tests

  **Acceptance Criteria**:
  - [ ] Module exports work
  - [ ] Events emitted correctly
  - [ ] `cargo test audio` passes

  **QA Scenarios**:
  ```
  Scenario: Full audio pipeline
    Tool: Bash
    Steps:
      1. Start recording
      2. Wait 3 seconds
      3. Stop and encode
      4. Verify WAV output
    Expected Result: Valid WAV file ~100KB
    Evidence: .sisyphus/evidence/plan2-03-pipeline.log
  ```

  **Commit**: YES
  - Message: `feat: add audio capture and encoding modules`
  - Files: `src-tauri/src/audio/*.rs`

- [x] 4. **Tauri Commands for Audio**

  **What to do**:
  - Add Tauri commands in `lib.rs`:
    - `#[tauri::command] async fn start_audio_recording() -> Result<()>`
    - `#[tauri::command] async fn stop_audio_recording() -> Result<Vec<u8>>`
  - Use `tokio::sync::Mutex` for thread-safe recorder state
  - Emit events to frontend
  - Write tests

  **Acceptance Criteria**:
  - [ ] Commands callable from frontend
  - [ ] Events emitted
  - [ ] Tests pass

  **QA Scenarios**:
  ```
  Scenario: Tauri audio commands work
    Tool: Bash
    Steps:
      1. Call start_audio_recording
      2. Wait 2 seconds
      3. Call stop_audio_recording
      4. Verify WAV bytes returned
    Expected Result: WAV data returned
    Evidence: .sisyphus/evidence/plan2-04-commands.log
  ```

  **Commit**: YES
  - Message: `feat: add tauri commands for audio recording`

---

## Final Verification

- [x] F1. **Audio Capture Test**
  Run `cargo test audio::recorder`. All tests must pass.
  Output: `Tests [8/8 pass]`

- [x] F2. **Audio Encoding Test**
  Run `cargo test audio::encoder`. All tests must pass.
  Output: `Tests [4/4 pass]`

- [x] F3. **WAV Validation**
  Generate WAV file and validate with `file` command.
  Output: `WAV [valid]` - Tests verify RIFF/WAVE headers

---

## Success Criteria

```bash
cargo test audio  # Expected: all tests pass
```

### Final Checklist
- [x] Audio capture works
- [x] WAV encoding works
- [x] 10-minute limit enforced
- [x] Tauri commands work
- [x] Events emitted

---

## Handoff Notes

Ce plan fournit:
- `AudioRecorder` - pour capture audio
- `AudioEncoder` - pour encoding WAV
- Tauri commands: `start_audio_recording`, `stop_audio_recording`
- Events: `audio-recording-started`, `audio-recording-stopped`, `audio-data-ready`

Le Plan 3 (Groq+Input) utilisera les WAV bytes pour la transcription.