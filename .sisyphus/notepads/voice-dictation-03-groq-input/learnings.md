# Active Window Detection Implementation Learnings

## X11 EWMH Implementation Details

- **_NET_ACTIVE_WINDOW**: Retrieved from root window property, returns window ID of active window
- **Window Title**: Priority order - `_NET_WM_NAME` (UTF-8) then `WM_NAME` (STRING) as fallback  
- **Window Class**: Retrieved via `WM_CLASS` property which contains two null-terminated strings (instance and class)
- **Error Handling**: All X11 operations can fail due to connection issues, missing atoms, or invalid window IDs

## x11rb Usage Patterns

- Use `x11rb::connect(None)` to connect to default X11 display
- Atom retrieval requires `intern_atom()` followed by `.reply()`
- Property retrieval uses `get_property()` with type specification
- Always handle UTF-8 decoding errors when converting property values to strings

## Platform Limitations

- X11-only implementation - will not work on Wayland or other display servers
- Requires proper X11 environment with EWMH-compliant window manager
- Integration tests must be marked as `[ignore]` since they require actual X11 display

## Error Handling Strategy

- Wrap all X11 operations in `AppError::InputError` for consistent error handling
- Provide descriptive error messages that include the specific X11 operation that failed
- Handle edge cases like zero window IDs and empty property values gracefully

## Testing Considerations

- Unit tests for data structures work fine without X11
- Integration tests require actual X11 display and should be ignored in CI
- Mocking X11 connections would be ideal but requires significant setup complexity

## Global Shortcut Implementation Learnings

### Platform Limitations
- tauri-plugin-global-shortcut v2 has limited platform support
- Currently works reliably on X11 Linux systems
- Wayland support is not available in current version
- Windows and macOS support may be limited or require additional configuration

### API Design Considerations
- Global shortcut registration requires access to Tauri AppHandle
- Implemented global state pattern with init() function to store AppHandle
- Callback functions use Box<dyn Fn(&str) + Send + Sync> for flexibility
- Events are emitted using Tauri's Emitter trait: "shortcut-pressed" and "shortcut-released"

### Error Handling
- All errors wrapped in AppError::InputError variant
- Proper lock error handling for global state access
- Clear error messages for initialization and registration failures

### Dependencies
- Added once_cell = "1.19" for lazy static initialization
- Uses existing tauri-plugin-global-shortcut = "2.0.0" dependency

### Testing Approach
- Unit tests verify compilation and type correctness
- Full integration testing requires running Tauri application
- API structure validated through compilation checks

# Groq API Client Implementation Learnings

## Key Implementation Details

1. **Multipart Form Upload**: Used `reqwest::multipart::Form` to create proper multipart form data for audio file upload with correct MIME type (`audio/wav`) and filename.

2. **Error Handling**: Implemented comprehensive error handling for:
   - Network errors (timeouts, connection issues)
   - HTTP status codes (429 rate limits, 401 invalid API key, 400 bad requests, 5xx server errors)
   - JSON parsing errors
   - Multipart form creation errors

3. **Retry Logic**: Implemented exponential backoff retry logic with max 3 attempts (1s, 2s, 4s delays).

4. **Testing Strategy**: Used `wiremock` to create mock HTTP servers for testing the client without hitting the actual Groq API.

5. **API Endpoint**: Used the correct Groq API endpoint: `https://api.groq.com/openai/v1/audio/transcriptions`

6. **Model**: Used the specified model: `whisper-large-v3-turbo`

## Dependencies Confirmed

- `reqwest` with `multipart` feature is available in Cargo.toml
- `wiremock` is available in dev-dependencies
- `tokio` with full features is available

## Type Integration

- Properly integrated with existing `TranscriptionResult` from `crate::types`
- Used `AppError::GroqApiError` for all Groq-specific errors
- Returned `Result<T>` from `crate::error`

## Testing Approach

- Created unit tests using `wiremock` to mock the Groq API endpoint
- Tested successful transcription response
- Tested rate limit error handling
- Tested client creation

## Limitations Noted

- Groq API basic transcription response doesn't include duration or detected language
- Duration and language fields in `TranscriptionResult` are set to default values (0 and None)
- Caller should handle duration calculation if needed based on audio file metadata

## Text Insertion Module Implementation Learnings

### Enigo 0.3 Usage
- Enigo 0.3 requires importing the `Keyboard` trait to access keyboard simulation methods
- The `text()` method handles Unicode characters properly and is the recommended way to input text
- Individual key events are handled using the `key()` method with `Direction::Press`, `Direction::Click`, and `Direction::Release`
- Enigo 0.3 does not have a built-in `set_key_press_delay()` method, so delays must be implemented manually using `std::thread::sleep()`

### Clipboard Implementation
- Since the Tauri clipboard plugin requires an `AppHandle` which isn't available in this module context, system commands were used instead:
  - Windows: `clip`
  - macOS: `pbcopy`
  - Linux: `xclip -selection clipboard`
- This approach avoids adding new dependencies while maintaining cross-platform compatibility

### Error Handling
- All enigo operations return `InputError` which must be mapped to the application's `AppError::InputError`
- The clipboard fallback mechanism ensures reliability: if direct typing fails, the system falls back to copy+paste

### Unicode Support
- Enigo's `text()` method properly handles Unicode characters including emojis and international characters
- The manual character-by-character approach with delays ensures reliable input even for complex Unicode sequences

### Testing Considerations
- Unit tests for input simulation are limited since they can't safely simulate actual keyboard input in test environments
- Tests verify that functions don't panic and handle errors appropriately
- Integration testing would be needed to verify actual functionality

### Key Implementation Details
- Added 10ms delay between keystrokes for reliability
- Implemented proper Ctrl+V (Cmd+V on macOS) paste simulation
- Fallback logic ensures text insertion works even if direct typing fails
- Cross-platform clipboard support using system-native commands