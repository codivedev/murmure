# Issues - Voice Dictation Frontend

## 2026-03-18 Session Start

### Known Issues
1. **Backend Commands Missing**: Plans 2-3 not implemented, so audio/groq/input commands don't exist yet
   - Workaround: Create mock implementations in frontend utils

### Dependencies on Other Plans
- Plan 2 (Audio): Required for actual audio recording
- Plan 3 (Groq+Input): Required for transcription and text insertion
- Integration testing will need all plans complete