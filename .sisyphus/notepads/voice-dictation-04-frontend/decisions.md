# Decisions - Voice Dictation Frontend

## 2026-03-18 Session Start

### Architecture Decisions
1. **Frontend Framework**: React + TypeScript with Vite
2. **Styling**: TailwindCSS for utility-first CSS
3. **State Management**: React hooks (useRecording) for recording state machine
4. **Mock Strategy**: Create mock implementations for Tauri commands from Plans 2-3 until they're implemented

### Directory Structure
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

### Component Design
- **SetupWizard**: Multi-step form for first-run setup
- **Settings**: Configuration UI for shortcut, language, API key
- **Overlay**: Floating overlay for recording state feedback
- **useRecording**: Hook managing recording state machine