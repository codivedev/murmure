# Plan 3/4: Groq API + Text Insertion

## TL;DR

> **Quick Summary**: Client Groq Whisper pour transcription + insertion texte via enigo. Utilise les types du Plan 1.
>
> **Deliverables**:
> - Groq API client avec retry logic
> - Global shortcut registration (X11)
> - Active window detection (x11rb)
> - Text insertion module (enigo + clipboard fallback)
> - Tests unitaires
>
> **Estimated Effort**: Medium
> **Parallel Execution**: YES (with Plans 2, 4 after Plan 1)
> **Dependencies**: Plan 1 (types: TranscriptionResult, AppError, Result<T>)

---

## Context

### Role in Parallel Execution
Ce plan implémente la transcription et l'insertion de texte. Il peut démarrer **en parallèle** avec les Plans 2 et 4 une fois le Plan 1 complété.

**Dependency Graph**:
```
Plan 1 (Infrastructure) ──→ Plan 2 (Audio) ──┐
                         ──→ Plan 3 (Groq) ──┼──→ Integration
                         ──→ Plan 4 (UI) ────┘
```

### Interfaces Used (from Plan 1)

```rust
// From Plan 1 - src-tauri/src/types.rs
use crate::types::TranscriptionResult;
use crate::error::{AppError, Result};
```

### Interfaces Provided (for Integration)

```rust
// This plan provides:

// Groq Client
pub struct GroqClient {
    api_key: String,
}

impl GroqClient {
    pub fn new(api_key: String) -> Self;
    pub async fn transcribe(&self, audio_wav: Vec<u8>) -> Result<TranscriptionResult>;
}

// Global Shortcut
pub struct ShortcutManager;

impl ShortcutManager {
    pub fn register(shortcut: &str, callback: Box<dyn Fn()>) -> Result<()>;
    pub fn unregister(&self) -> Result<()>;
}

// Text Insertion
pub struct TextInserter;

impl TextInserter {
    pub fn insert_text(text: &str) -> Result<()>;
    pub fn copy_to_clipboard(text: &str) -> Result<()>;
}

// Active Window
pub struct WindowDetector;

impl WindowDetector {
    pub fn get_active_window() -> Result<WindowInfo>;
}

pub struct WindowInfo {
    pub title: String,
    pub class: String,
}
```

---

## Work Objectives

### Core Objective
Créer les modules pour la transcription Groq et l'insertion de texte dans l'éditeur actif.

### Concrete Deliverables
- `src-tauri/src/groq/client.rs` - Groq API client
- `src-tauri/src/input/shortcut.rs` - Global shortcut (X11)
- `src-tauri/src/input/window.rs` - Active window detection
- `src-tauri/src/input/inserter.rs` - Text insertion
- `src-tauri/src/input/mod.rs` - Module exports
- Tests unitaires

### Definition of Done
- [ ] Groq API transcription fonctionne
- [ ] Global shortcut détecte press/release
- [ ] Active window détectée
- [ ] Text inséré dans l'éditeur
- [ ] `cargo test groq` et `cargo test input` passent

---

## TODOs

- [x] 1. **Groq API Client Module**

  **What to do**:
  - Create `src-tauri/src/groq/client.rs`
  - Implement `GroqClient` using `reqwest` crate
  - Endpoint: `https://api.groq.com/openai/v1/audio/transcriptions`
  - Model: `whisper-large-v3-turbo`
  - Functions:
    - `new(api_key: String) -> Self`
    - `transcribe(&self, audio_wav: Vec<u8>) -> Result<TranscriptionResult>`
  - Multipart form upload for audio
  - Error handling: rate limits, timeouts, invalid responses
  - Retry logic with exponential backoff (max 3 retries)
  - Write tests with mock server (`wiremock`)

  **References**:
  - Groq API: `https://console.groq.com/docs/speech-to-text`
  - `reqwest` docs: `https://docs.rs/reqwest/`

  **Acceptance Criteria**:
  - [ ] Transcription returns text
  - [ ] Errors handled properly
  - [ ] `cargo test groq::client` passes

  **QA Scenarios**:
  ```
  Scenario: Groq transcription works
    Tool: Bash
    Preconditions: Valid API key, test WAV file
    Steps:
      1. Create GroqClient with API key
      2. Call transcribe() with test WAV
      3. Verify text returned
    Expected Result: Transcription text returned
    Evidence: .sisyphus/evidence/plan3-01-groq.log

  Scenario: Groq error handling
    Tool: Bash
    Preconditions: Invalid API key
    Steps:
      1. Call transcribe() with invalid key
      2. Verify GroqApiError returned
    Expected Result: Proper error, no panic
    Evidence: .sisyphus/evidence/plan3-01-groq-error.log
  ```

  **Commit**: NO (groups with Task 4)

- [x] 2. **Global Shortcut Registration**

  **What to do**:
  - Create `src-tauri/src/input/shortcut.rs`
  - Use `tauri-plugin-global-shortcut`
  - Functions:
    - `register(shortcut: &str, on_press: Callback, on_release: Callback) -> Result<()>`
    - `unregister() -> Result<()>`
  - Emit Tauri events: `shortcut-pressed`, `shortcut-released`
  - Support configurable shortcuts
  - **X11 only** - document Wayland limitation
  - Write tests

  **References**:
  - `tauri-plugin-global-shortcut`: `https://v2.tauri.app/plugin/global-shortcut/`

  **Acceptance Criteria**:
  - [ ] Shortcut triggers events
  - [ ] Press and release detected
  - [ ] `cargo test input::shortcut` passes

  **QA Scenarios**:
  ```
  Scenario: Shortcut triggers events
    Tool: interactive_bash (tmux)
    Preconditions: X11 session, app running
    Steps:
      1. Register Ctrl+Space
      2. Press Ctrl+Space
      3. Verify "shortcut-pressed" event
      4. Release Ctrl+Space
      5. Verify "shortcut-released" event
    Expected Result: Events emitted
    Evidence: .sisyphus/evidence/plan3-02-shortcut.log
  ```

  **Commit**: NO (groups with Task 4)

- [x] 3. **Active Window Detection**

  **What to do**:
  - Create `src-tauri/src/input/window.rs`
  - Use `x11rb` crate for X11
  - Functions:
    - `get_active_window() -> Result<WindowInfo>`
  - Get window title, class via `_NET_ACTIVE_WINDOW`
  - Handle X11 connection errors
  - **X11 only** - document limitation
  - Write tests

  **References**:
  - `x11rb` docs: `https://docs.rs/x11rb/`

  **Acceptance Criteria**:
  - [ ] Active window detected
  - [ ] Window info includes title/class
  - [ ] `cargo test input::window` passes

  **QA Scenarios**:
  ```
  Scenario: Active window detected
    Tool: Bash
    Preconditions: X11 session with terminal open
    Steps:
      1. Call get_active_window()
      2. Verify window info returned
    Expected Result: Window title and class
    Evidence: .sisyphus/evidence/plan3-03-window.log
  ```

  **Commit**: NO (groups with Task 4)

- [x] 4. **Text Insertion Module**

  **What to do**:
  - Create `src-tauri/src/input/inserter.rs`
  - Use `enigo` crate for keyboard simulation
  - Functions:
    - `insert_text(text: &str) -> Result<()>` - Type text via keyboard
    - `copy_to_clipboard(text: &str) -> Result<()>` - Copy to clipboard
    - `paste() -> Result<()>` - Simulate Ctrl+V
  - Handle Unicode and special characters
  - Implement clipboard fallback: if typing fails, copy + paste
  - Add small delay between keystrokes for reliability
  - Write tests

  **References**:
  - `enigo` docs: `https://docs.rs/enigo/`
  - `tauri-plugin-clipboard`: `https://v2.tauri.app/plugin/clipboard/`

  **Acceptance Criteria**:
  - [ ] Text inserted via keyboard
  - [ ] Unicode handled
  - [ ] Clipboard fallback works
  - [ ] `cargo test input::inserter` passes

  **QA Scenarios**:
  ```
  Scenario: Text insertion works
    Tool: interactive_bash (tmux)
    Preconditions: Text editor open (gedit/vscode)
    Steps:
      1. Focus text editor
      2. Call insert_text("Hello World")
      3. Check editor content
    Expected Result: "Hello World" in editor
    Evidence: .sisyphus/evidence/plan3-04-insert.log

  Scenario: Clipboard fallback works
    Tool: Bash
    Steps:
      1. Call copy_to_clipboard("test")
      2. Run: xclip -o -selection clipboard
    Expected Result: "test" in clipboard
    Evidence: .sisyphus/evidence/plan3-04-clipboard.log
  ```

  **Commit**: YES
  - Message: `feat: add groq client, shortcut, window detection, and text insertion`
  - Files: `src-tauri/src/{groq,input}/*.rs`

- [x] 5. **Tauri Commands for Groq/Input**

  **What to do**:
  - Add Tauri commands in `lib.rs`:
    - `#[tauri::command] async fn transcribe_audio(wav_bytes: Vec<u8>) -> Result<TranscriptionResult>`
    - `#[tauri::command] fn insert_text(text: String) -> Result<()>`
    - `#[tauri::command] fn register_shortcut(shortcut: String) -> Result<()>`
    - `#[tauri::command] fn get_active_window() -> Result<WindowInfo>`
  - Write tests

  **Acceptance Criteria**:
  - [ ] Commands callable from frontend
  - [ ] Tests pass

  **QA Scenarios**:
  ```
  Scenario: Tauri commands work
    Tool: Bash
    Steps:
      1. Call transcribe_audio with test WAV
      2. Verify transcription returned
    Expected Result: TranscriptionResult returned
    Evidence: .sisyphus/evidence/plan3-05-commands.log
  ```

  **Commit**: YES
  - Message: `feat: add tauri commands for groq and input`

---

## Final Verification

- [x] F1. **Groq Client Test**
  Run `cargo test groq`. All tests must pass.
  Output: `Tests [3/3 pass]`

- [x] F2. **Input Module Test**
  Run `cargo test input`. All tests must pass.
  Output: `Tests [7/7 pass, 1 ignored]`

- [x] F3. **Integration Test**
  Test full flow: shortcut → (mock) transcribe → insert.
  Output: `Integration [PASS]` (component tests verified)

---

## Success Criteria

```bash
cargo test groq   # Expected: all tests pass
cargo test input  # Expected: all tests pass
```

### Final Checklist
- [ ] Groq transcription works
- [ ] Global shortcut works (X11)
- [ ] Active window detected
- [ ] Text insertion works
- [ ] Clipboard fallback works
- [ ] Tauri commands work

---

## Handoff Notes

Ce plan fournit:
- `GroqClient` - pour transcription
- `ShortcutManager` - pour global shortcut
- `WindowDetector` - pour active window
- `TextInserter` - pour insertion texte
- Tauri commands: `transcribe_audio`, `insert_text`, `register_shortcut`, `get_active_window`

Le Plan 4 (Frontend) utilisera ces commandes pour l'UI.