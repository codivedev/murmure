# Changelog

All notable changes to Murmure will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - Initial Release

### Added

- Voice-to-text transcription using Groq Whisper API
- Global shortcut activation (configurable, default: Ctrl+Space)
- System tray integration with quick actions
- Setup wizard for first-time configuration
- Settings management UI
- Auto-copy transcription to clipboard
- Language selection for transcription

### Technical

- Tauri v2 backend
- React frontend with TypeScript
- TailwindCSS styling
- Rust system tray implementation
- X11 global shortcut support

### Known Issues

- Audio module incomplete (Plans 2-3 not implemented)
- X11 only - Wayland not supported for global shortcuts
- Mock implementations for audio commands in frontend

### Requirements

- Linux (X11)
- Node.js 18+
- Rust (latest stable)
- Groq API key
