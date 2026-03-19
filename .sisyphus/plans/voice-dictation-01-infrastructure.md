# Plan 1/4: Infrastructure & Foundation

## TL;DR

> **Quick Summary**: Foundation du projet - scaffolding, types partagés, error handling, settings storage. Ce plan DOIT être complété en premier car les autres plans dépendent de ses outputs.
>
> **Deliverables**:
> - Projet Tauri v2 initialisé
> - Types/interfaces partagés (AudioConfig, TranscriptionResult, AppError, Settings)
> - Error handling framework
> - Settings storage layer
> - API key encryption utility
> - Tests infrastructure
>
> **Estimated Effort**: Quick
> **Parallel Execution**: NO (foundation for other plans)
> **Critical Path**: This plan → Plans 2, 3, 4 can start

---

## Context

### Role in Parallel Execution
Ce plan est le **foundation**. Il définit les types et interfaces que les 3 autres plans utiliseront.

**Dependency Graph**:
```
Plan 1 (Infrastructure) ──→ Plan 2 (Audio)
                         ──→ Plan 3 (Groq+Input)
                         ──→ Plan 4 (Frontend)
```

### Shared Contracts (Outputs for Other Plans)

Ces types seront utilisés par les autres plans :

```rust
// src-tauri/src/types.rs

/// Audio configuration - Used by Plan 2 (Audio)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,  // 16000
    pub channels: u16,     // 1 (mono)
    pub sample_format: SampleFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Error types - Used by all plans
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Audio error: {0}")]
    AudioError(String),
    
    #[error("Groq API error: {0}")]
    GroqApiError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Input error: {0}")]
    InputError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

---

## Work Objectives

### Core Objective
Créer la foundation technique du projet avec tous les types, interfaces et utilitaires partagés.

### Concrete Deliverables
- Projet Tauri v2 initialisé avec structure de dossiers
- Fichier `types.rs` avec tous les types partagés
- Module `error.rs` avec AppError et Result<T>
- Module `config.rs` avec SettingsStore
- Module `crypto.rs` avec API key encryption
- Tests unitaires pour tous les modules

### Definition of Done
- [x] `cargo tauri dev` lance une fenêtre vide
- [x] Tous les types compilent sans erreurs
- [x] `cargo test` passe
- [x] Les autres plans peuvent utiliser ces types

---

## TODOs

- [x] 1. **Project Scaffolding Tauri v2**

  **What to do**:
  - Initialize Tauri v2 project: `cargo tauri init`
  - Configure `tauri.conf.json` for Linux X11
  - Create directory structure:
    ```
    src-tauri/
    ├── src/
    │   ├── main.rs
    │   ├── lib.rs
    │   ├── types.rs      (shared types)
    │   ├── error.rs      (error types)
    │   ├── config.rs     (settings storage)
    │   ├── crypto.rs     (API key encryption)
    │   └── audio/        (placeholder for Plan 2)
    │   └── groq/         (placeholder for Plan 3)
    │   └── input/        (placeholder for Plan 3)
    ```
  - Add dependencies to Cargo.toml:
    - `tauri`, `tauri-plugin-global-shortcut`, `tauri-plugin-clipboard`, `tauri-plugin-store`
    - `serde`, `serde_json`
    - `thiserror`, `anyhow`
    - `tokio` (async runtime)
    - `keyring` (API key storage)

  **Acceptance Criteria**:
  - [ ] `cargo tauri dev` launches empty window
  - [ ] Directory structure created
  - [ ] All dependencies in Cargo.toml

  **QA Scenarios**:
  ```
  Scenario: Project builds
    Tool: Bash
    Steps:
      1. cargo tauri dev
    Expected Result: Window appears, no errors
    Evidence: .sisyphus/evidence/plan1-01-build.log
  ```

  **Commit**: YES
  - Message: `feat: initialize tauri v2 project with shared types structure`

- [x] 2. **Shared Types Definition**

  **What to do**:
  - Create `src-tauri/src/types.rs` with all shared types
  - Define: AudioConfig, TranscriptionResult, AppState, Settings, OverlayPosition
  - Add `#[derive(Serialize, Deserialize)]` for all types
  - Add `#[derive(Debug, Clone, PartialEq)]` where appropriate
  - Write unit tests for serialization

  **Acceptance Criteria**:
  - [ ] All types defined
  - [ ] `cargo test types` passes
  - [ ] Types accessible from other modules

  **QA Scenarios**:
  ```
  Scenario: Types serialize correctly
    Tool: Bash
    Steps:
      1. cargo test types::serialization
    Expected Result: All tests pass
    Evidence: .sisyphus/evidence/plan1-02-types.log
  ```

  **Commit**: NO (groups with Task 1)

- [x] 3. **Error Types Framework**

  **What to do**:
  - Create `src-tauri/src/error.rs`
  - Define `AppError` enum with variants: AudioError, GroqApiError, ConfigError, InputError, NetworkError, IoError
  - Implement `From` traits for underlying errors
  - Create `Result<T>` type alias
  - Write tests for error conversion

  **Acceptance Criteria**:
  - [ ] All error types defined
  - [ ] `From` traits implemented
  - [ ] `cargo test error` passes

  **QA Scenarios**:
  ```
  Scenario: Error conversion works
    Tool: Bash
    Steps:
      1. cargo test error::conversion
    Expected Result: All tests pass
    Evidence: .sisyphus/evidence/plan1-03-error.log
  ```

  **Commit**: NO (groups with Task 1)

- [x] 4. **Settings Storage Layer**

  **What to do**:
  - Create `src-tauri/src/config.rs`
  - Implement `SettingsStore` using `tauri-plugin-store`
  - Functions: `load_settings()`, `save_settings()`, `reset_settings()`
  - Default values: shortcut="Ctrl+Space", language="auto"
  - Write tests for persistence

  **Acceptance Criteria**:
  - [ ] Settings persist across restarts
  - [ ] Default values applied
  - [ ] `cargo test config` passes

  **QA Scenarios**:
  ```
  Scenario: Settings persist
    Tool: Bash
    Steps:
      1. Save settings
      2. Restart app
      3. Load settings
    Expected Result: Settings unchanged
    Evidence: .sisyphus/evidence/plan1-04-config.log
  ```

  **Commit**: NO (groups with Task 1)

- [x] 5. **API Key Encryption Utility**

  **What to do**:
  - Create `src-tauri/src/crypto.rs`
  - Use `keyring` crate for OS keychain storage
  - Functions: `store_api_key()`, `retrieve_api_key()`, `delete_api_key()`
  - Add fallback encryption if keychain unavailable
  - Write tests for roundtrip

  **Acceptance Criteria**:
  - [ ] API key stored securely
  - [ ] Fallback works
  - [ ] `cargo test crypto` passes

  **QA Scenarios**:
  ```
  Scenario: API key roundtrip
    Tool: Bash
    Steps:
      1. Store key "test-123"
      2. Retrieve key
      3. Verify match
    Expected Result: Keys match
    Evidence: .sisyphus/evidence/plan1-05-crypto.log
  ```

  **Commit**: YES
  - Message: `feat: add shared types, error handling, settings, and crypto modules`
  - Files: `src-tauri/src/{types,error,config,crypto}.rs`

- [x] 6. **Tauri Commands for Settings**

  **What to do**:
  - Add Tauri commands in `lib.rs`:
    - `#[tauri::command] fn get_settings() -> Result<Settings>`
    - `#[tauri::command] fn save_settings(settings: Settings) -> Result<()>`
    - `#[tauri::command] fn store_api_key(key: String) -> Result<()>`
    - `#[tauri::command] fn has_api_key() -> Result<bool>`
  - Register commands in `main.rs`
  - Write integration tests

  **Acceptance Criteria**:
  - [ ] Commands callable from frontend
  - [ ] All commands return proper types
  - [ ] Tests pass

  **QA Scenarios**:
  ```
  Scenario: Tauri commands work
    Tool: Bash
    Steps:
      1. Call get_settings from frontend
      2. Verify response
    Expected Result: Settings returned
    Evidence: .sisyphus/evidence/plan1-06-commands.log
  ```

  **Commit**: YES
  - Message: `feat: add tauri commands for settings and api key`

---

## Final Verification

- [x] F1. **Build Check**
  Run `cargo build` and `cargo test`. All must pass.
  Output: `Build [PASS/FAIL] | Tests [N/N]`

- [x] F2. **Types Export Check**
  Verify all types are exported and usable from other modules.
  Output: `Types [N/N exported]`

---

## Success Criteria

```bash
cargo build   # Expected: success
cargo test    # Expected: all tests pass
```

### Final Checklist
- [x] Project builds
- [x] All types defined and exported
- [x] Error handling works
- [x] Settings persist
- [x] API key encryption works
- [x] Tauri commands callable

---

## Handoff to Other Plans

Une fois ce plan complété, les autres plans peuvent démarrer :

- **Plan 2 (Audio)**: Utilise `AudioConfig`, `AppError`, `Result<T>`
- **Plan 3 (Groq+Input)**: Utilise `TranscriptionResult`, `AppError`, `Result<T>`
- **Plan 4 (Frontend)**: Utilise `AppState`, `Settings`, Tauri commands