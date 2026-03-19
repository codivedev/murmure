# Plan 4/4: Frontend UI

## TL;DR

> **Quick Summary**: Interface utilisateur React/TypeScript - setup wizard, settings, overlay, tray icon, orchestration. Utilise les Tauri commands des Plans 1-3.
>
> **Deliverables**:
> - Frontend React/TypeScript avec TailwindCSS
> - Setup wizard (first-run)
> - Settings UI
> - Overlay component
> - Tray icon integration
> - Main app orchestration
> - Tests et documentation
>
> **Estimated Effort**: Medium
> **Parallel Execution**: YES (with Plans 2, 3 after Plan 1)
> **Dependencies**: Plan 1 (types, commands), Plan 2 (audio commands), Plan 3 (groq/input commands)

---

## Context

### Role in Parallel Execution
Ce plan implémente l'interface utilisateur. Il peut démarrer **en parallèle** avec les Plans 2 et 3 une fois le Plan 1 complété, mais l'intégration finale nécessite que tous les plans soient complétés.

**Dependency Graph**:
```
Plan 1 (Infrastructure) ──→ Plan 2 (Audio) ──┐
                         ──→ Plan 3 (Groq) ──┼──→ Plan 4 Integration
                         ──→ Plan 4 (UI) ────┘
```

### Interfaces Used (from Plans 1-3)

```typescript
// Tauri commands from Plan 1
invoke('get_settings'): Promise<Settings>
invoke('save_settings', { settings }): Promise<void>
invoke('store_api_key', { key }): Promise<void>
invoke('has_api_key'): Promise<boolean>

// Tauri commands from Plan 2
invoke('start_audio_recording'): Promise<void>
invoke('stop_audio_recording'): Promise<Vec<u8>>

// Tauri commands from Plan 3
invoke('transcribe_audio', { wavBytes }): Promise<TranscriptionResult>
invoke('insert_text', { text }): Promise<void>
invoke('register_shortcut', { shortcut }): Promise<void>
invoke('get_active_window'): Promise<WindowInfo>

// Types from Plan 1
interface Settings {
  shortcut: string;
  language: string;
  overlayPosition: { x: number; y: number };
  setupCompleted: boolean;
}

interface TranscriptionResult {
  text: string;
  durationMs: number;
  language?: string;
}

// Events from Plans 2-3
listen('shortcut-pressed'): Promise<void>
listen('shortcut-released'): Promise<void>
listen('audio-recording-started'): Promise<void>
listen('audio-recording-stopped'): Promise<{ durationMs: number }>
```

---

## Work Objectives

### Core Objective
Créer une interface utilisateur intuitive et réactive pour l'application de dictée vocale.

### Concrete Deliverables
- `src/` - Frontend React/TypeScript
- `src/components/SetupWizard.tsx` - First-run setup
- `src/components/Settings.tsx` - Settings UI
- `src/components/Overlay.tsx` - Recording overlay
- `src/hooks/useRecording.ts` - Recording state hook
- `src/App.tsx` - Main app orchestration
- `README.md` - Documentation

### Definition of Done
- [ ] Setup wizard s'affiche au premier lancement
- [ ] Settings UI permet de changer raccourci
- [ ] Overlay s'affiche pendant enregistrement
- [ ] Full flow fonctionne: shortcut → record → transcribe → insert
- [ ] `npm run build` réussit
- [ ] Tests passent

---

## TODOs

- [x] 1. **Frontend Project Setup**

  **What to do**:
  - Initialize React + TypeScript in `src/`
  - Configure Vite for Tauri
  - Set up TailwindCSS
  - Create directory structure:
    ```
    src/
    ├── main.tsx
    ├── App.tsx
    ├── components/
    │   ├── SetupWizard.tsx
    │   ├── Settings.tsx
    │   └── Overlay.tsx
    ├── hooks/
    │   └── useRecording.ts
    ├── utils/
    │   └── tauri.ts
    └── styles/
        └── index.css
    ```
  - Add dependencies: `@tauri-apps/api`, `@tauri-apps/plugin-global-shortcut`, `@tauri-apps/plugin-clipboard`

  **Acceptance Criteria**:
  - [ ] `npm run dev` starts Vite
  - [ ] TailwindCSS works
  - [ ] Tauri API imports resolve

  **QA Scenarios**:
  ```
  Scenario: Frontend builds
    Tool: Bash
    Steps:
      1. npm run build
    Expected Result: Build succeeds
    Evidence: .sisyphus/evidence/plan4-01-build.log
  ```

  **Commit**: NO (groups with Task 2)

- [x] 2. **Setup Wizard Component**

  **What to do**:
  - Create `src/components/SetupWizard.tsx`
  - Multi-step form:
    - Step 1: Welcome screen
    - Step 2: API key input (validate format)
    - Step 3: Microphone test (call `start_audio_recording` briefly)
    - Step 4: Shortcut configuration
    - Step 5: Complete
  - Store API key via `store_api_key`
  - Mark setup completed in settings
  - Style with TailwindCSS

  **Acceptance Criteria**:
  - [ ] Wizard shows on first launch
  - [ ] API key validated
  - [ ] Setup completion saved

  **QA Scenarios**:
  ```
  Scenario: Setup wizard completes
    Tool: Playwright
    Preconditions: Fresh install
    Steps:
      1. Launch app
      2. Enter API key
      3. Complete steps
      4. Verify main UI shown
    Expected Result: Wizard completes
    Evidence: .sisyphus/evidence/plan4-02-wizard.png
  ```

  **Commit**: NO (groups with Task 2)

- [x] 3. **Settings Component**

  **What to do**:
  - Create `src/components/Settings.tsx`
  - Sections:
    - Shortcut: recorder that captures key combination
    - Language: dropdown (Auto, English, French, etc.)
    - API Key: masked display + change button
    - About: version, links
  - Save/Reset buttons
  - Load settings on mount via `get_settings`
  - Save via `save_settings`

  **Acceptance Criteria**:
  - [ ] Shortcut can be changed
  - [ ] Language can be selected
  - [ ] Settings persist

  **QA Scenarios**:
  ```
  Scenario: Change shortcut
    Tool: Playwright
    Steps:
      1. Open settings
      2. Click shortcut recorder
      3. Press Ctrl+Shift+V
      4. Save
      5. Verify shortcut updated
    Expected Result: Shortcut changed
    Evidence: .sisyphus/evidence/plan4-03-settings.png
  ```

  **Commit**: NO (groups with Task 2)

- [x] 4. **Overlay Component**

  **What to do**:
  - Create `src/components/Overlay.tsx`
  - States:
    - Hidden (default)
    - Recording: "Recording... 00:03" with timer
    - Processing: spinner + "Transcribing..."
    - Success: show text briefly (2s)
    - Error: show error (5s)
  - Transparent background with blur
  - Centered on screen
  - CSS animations for show/hide

  **Acceptance Criteria**:
  - [ ] Overlay shows on recording
  - [ ] Timer updates
  - [ ] Result shown briefly

  **QA Scenarios**:
  ```
  Scenario: Overlay shows recording
    Tool: Playwright
    Steps:
      1. Press shortcut
      2. Wait 3 seconds
      3. Verify overlay visible
    Expected Result: "Recording... 00:03"
    Evidence: .sisyphus/evidence/plan4-04-overlay.png
  ```

  **Commit**: YES
  - Message: `feat: add setup wizard, settings, and overlay components`
  - Files: `src/components/*.tsx, src/styles/*.css`

- [x] 5. **Recording Hook**

  **What to do**:
  - Create `src/hooks/useRecording.ts`
  - State machine: Idle → Recording → Processing → Success/Error → Idle
  - Listen to events: `shortcut-pressed`, `shortcut-released`
  - On press: call `start_audio_recording`
  - On release: call `stop_audio_recording`, then `transcribe_audio`, then `insert_text`
  - Handle errors gracefully
  - Return: `{ state, transcription, error, start, stop }`

  **Acceptance Criteria**:
  - [ ] State transitions correctly
  - [ ] Full flow works
  - [ ] Errors handled

  **QA Scenarios**:
  ```
  Scenario: Recording hook flow
    Tool: Bash
    Steps:
      1. Trigger shortcut press
      2. Wait 2 seconds
      3. Trigger shortcut release
      4. Verify transcription inserted
    Expected Result: Text inserted
    Evidence: .sisyphus/evidence/plan4-05-hook.log
  ```

  **Commit**: NO (groups with Task 6)

- [x] 6. **Main App Orchestration**

  **What to do**:
  - Update `src/App.tsx`
  - Check `has_api_key` on mount
  - If no API key: show SetupWizard
  - If API key exists: show main UI (hidden window, overlay only)
  - Register shortcut via `register_shortcut`
  - Set up event listeners
  - Initialize tray icon (via Tauri config)
  - Handle window visibility

  **Acceptance Criteria**:
  - [ ] App starts correctly
  - [ ] Setup wizard shown when needed
  - [ ] Full flow works

  **QA Scenarios**:
  ```
  Scenario: Full end-to-end flow
    Tool: interactive_bash (tmux)
    Preconditions: API key configured, text editor open
    Steps:
      1. Launch app
      2. Focus text editor
      3. Press shortcut
      4. Say "hello world"
      5. Release shortcut
      6. Wait 5 seconds
      7. Check text editor
    Expected Result: Transcription in editor
    Evidence: .sisyphus/evidence/plan4-06-e2e.log
  ```

  **Commit**: YES
  - Message: `feat: add main app orchestration with recording hook`
  - Files: `src/App.tsx, src/hooks/*.ts`

- [x] 7. **Tray Icon Integration**

  **What to do**:
  - Configure tray icon in `tauri.conf.json`
  - Menu items: Show Settings, Quit
  - Icon changes during recording (optional)
  - Handle tray events in Rust

  **Acceptance Criteria**:
  - [ ] Tray icon appears
  - [ ] Menu works

  **QA Scenarios**:
  ```
  Scenario: Tray menu works
    Tool: Bash
    Steps:
      1. Right-click tray icon
      2. Click Quit
    Expected Result: App closes
    Evidence: .sisyphus/evidence/plan4-07-tray.log
  ```

  **Commit**: NO (groups with Task 8)

- [x] 8. **Error Handling UI**

  **What to do**:
  - Add error display in Overlay
  - User-friendly messages:
    - "Microphone not available"
    - "No internet connection"
    - "Transcription failed"
    - "Rate limited, please wait"
  - Show for 5 seconds then hide
  - Log errors to console

  **Acceptance Criteria**:
  - [ ] Errors shown to user
  - [ ] App doesn't crash

  **QA Scenarios**:
  ```
  Scenario: Error shown
    Tool: Bash
    Preconditions: No internet
    Steps:
      1. Complete recording
      2. Verify error shown
    Expected Result: "No internet connection"
    Evidence: .sisyphus/evidence/plan4-08-error.png
  ```

  **Commit**: YES
  - Message: `feat: add tray icon and error handling UI`

- [x] 9. **Documentation**

  **What to do**:
  - Create `README.md`:
    - Project description
    - Installation (Pop!_OS/Ubuntu)
    - Usage guide
    - Configuration
    - Troubleshooting
    - Development setup
  - Create `CHANGELOG.md`
  - Add `LICENSE` (MIT)

  **Acceptance Criteria**:
  - [ ] README complete
  - [ ] CHANGELOG initialized

  **QA Scenarios**:
  ```
  Scenario: Docs exist
    Tool: Bash
    Steps:
      1. Check README.md exists
      2. Check LICENSE exists
    Expected Result: Files exist
    Evidence: .sisyphus/evidence/plan4-09-docs.log
  ```

  **Commit**: YES
  - Message: `docs: add readme, changelog, and license`

---

## Final Verification

- [x] F1. **Frontend Build**
  Run `npm run build`. Must succeed.
  Output: `Build [PASS]`

- [x] F2. **Full E2E Test**
  Test complete flow: shortcut → record → transcribe → insert.
  Output: `E2E [PASS - Mock mode]`

- [x] F3. **UI Review**
  Verify all UI components render correctly.
  Output: `UI [4/4 components]`

---

## Success Criteria

```bash
npm run build    # Expected: success
cargo tauri dev  # Expected: app launches, full flow works
```

### Final Checklist
- [x] Setup wizard works
- [x] Settings UI works
- [x] Overlay shows correctly
- [x] Full flow works (mock mode - Plans 2-3 pending)
- [x] Tray icon works
- [x] Error handling works
- [x] Documentation complete

---

## Integration Notes

Une fois tous les plans complétés, l'intégration finale consiste à:

1. Vérifier que tous les modules se compile ensemble
2. Tester le flow end-to-end complet
3. Optimiser les performances
4. Finaliser la documentation

Le plan d'intégration sera créé après que tous les plans parallèles soient complétés.