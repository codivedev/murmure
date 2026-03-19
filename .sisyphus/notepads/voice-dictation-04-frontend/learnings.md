# Learnings - Voice Dictation Frontend

## 2026-03-18 Session Start

### Project Context
- Tauri v2 application for voice dictation
- Backend (Plans 1-3) partially implemented:
  - Plan 1 (Infrastructure): COMPLETE - types, error handling, settings, crypto
  - Plan 2 (Audio): NOT STARTED - audio capture/encoding modules empty
  - Plan 3 (Groq+Input): NOT STARTED - groq/input modules empty

### Available Tauri Commands (from Plan 1)
- `get_settings()` -> Settings
- `save_settings(settings: Settings)` -> void
- `store_api_key(key: String)` -> void
- `has_api_key()` -> boolean

### Types Available (from Plan 1)
```typescript
interface Settings {
  shortcut: string;
  language: string;
  overlay_position: { x: number; y: number };
  setup_completed: boolean;
}

interface TranscriptionResult {
  text: string;
  duration_ms: number;
  language?: string;
}

interface AppState {
  type: 'Idle' | 'Recording' | 'Processing' | 'Success' | 'Error';
  data?: { duration_ms?: number; text?: string; message?: string };
}
```

### Commands Expected from Plans 2-3 (NOT YET IMPLEMENTED)
- `start_audio_recording()` -> void
- `stop_audio_recording()` -> Vec<u8> (WAV bytes)
- `transcribe_audio(wav_bytes: Vec<u8>)` -> TranscriptionResult
- `insert_text(text: String)` -> void
- `register_shortcut(shortcut: String)` -> void
- `get_active_window()` -> WindowInfo

### Events Expected from Plans 2-3
- `shortcut-pressed`
- `shortcut-released`
- `audio-recording-started`
- `audio-recording-stopped` { duration_ms: number }

## Settings Component Implementation (2026-03-18)

### Patterns Used

1. **State Management**: Used a single state object with useState for all component state, similar to SetupWizard pattern
2. **Async Loading**: Implemented loading state with spinner while fetching initial settings
3. **Form Handling**: Text inputs and select dropdowns with onChange handlers updating state
4. **Conditional Rendering**: Show/hide API key input form based on user interaction
5. **Feedback UI**: Success/error messages with auto-dismiss after 3 seconds
6. **Section Organization**: Used visual dividers and icon headers for each settings section

### Tailwind Patterns from SetupWizard

- Consistent color scheme: blue-600 for primary actions, gray for secondary
- Icon containers: 10x10 or 12x12 rounded-lg with colored backgrounds
- Form inputs: border-gray-300, focus:ring-2 focus:ring-blue-500
- Buttons: rounded-lg, transition-colors duration-200
- Status indicators: colored backgrounds with icons (green-50 for success, red-50 for error)

### Component Structure

- Loading state with spinner
- Four main sections: Shortcut, Language, API Key, About
- Action buttons at bottom: Reset and Save
- Success/error feedback at top of component
## Overlay Component Implementation (2026-03-18)

### Design System Patterns Observed

#### Color Palette
- Primary: `blue-600` for buttons and active states
- Success: `green-500/600` with `green-50/100` backgrounds
- Error: `red-500/600` with `red-50/100` backgrounds
- Neutral: `gray-100` to `gray-900` scale
- Overlay background: `bg-gray-900/80` with `backdrop-blur-sm`

#### Spacing
- Standard padding: `p-6` for cards, `px-8 py-6` for overlay
- Border radius: `rounded-lg` (cards), `rounded-2xl` (overlay), `rounded-full` (icons)
- Icon containers: `w-10 h-10` or `w-12 h-12`

#### Typography
- Headings: `text-xl font-semibold` or `text-lg font-medium`
- Body: `text-gray-600` or `text-gray-500`
- Monospace for timers: `font-mono`

#### Animation Patterns
- Spinners: `animate-spin` with border technique
- Pulses: `animate-pulse` and `animate-ping` for recording indicator
- Transitions: `transition-colors duration-200` for buttons

### Component Implementation

#### Overlay States
1. **Idle**: Returns `null` (not rendered)
2. **Recording**: Shows pulsing red dot + timer (MM:SS format)
3. **Processing**: Shows spinner + "Transcribing..." text
4. **Success**: Green checkmark + truncated text (max 60 chars)
5. **Error**: Red X icon + error message

#### Key Implementation Details
- Uses `fixed inset-0` for full-screen overlay positioning
- Centered with `flex items-center justify-center`
- High z-index `z-50` to stay on top
- Semi-transparent dark background with blur effect
- CSS animations via Tailwind classes (no external libraries)
- Timer updates every second using `useEffect` + `setInterval`

### AppState Type
Located in `src/utils/tauri.ts`:
```typescript
export type AppState =
  | { type: 'Idle' }
  | { type: 'Recording'; data: { duration_ms: number } }
  | { type: 'Processing' }
  | { type: 'Success'; data: { text: string } }
  | { type: 'Error'; data: { message: string } };
```
