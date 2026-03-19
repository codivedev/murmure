# Murmure

Voice dictation application using Groq Whisper API.

## Features

- Voice-to-text transcription via Groq Whisper API
- Global shortcut activation (default: Ctrl+Space)
- System tray integration
- Setup wizard for first-time users
- Settings management

## Installation (Pop!_OS/Ubuntu)

### Prerequisites

- Node.js 18+ and npm
- Rust (latest stable)
- Tauri CLI

### Setup

1. Install system dependencies:

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

2. Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Install Tauri CLI:

```bash
npm install -g @tauri-apps/cli
```

4. Clone and install dependencies:

```bash
git clone <repository-url>
cd murmure2
npm install
```

## Usage

### First-Time Setup

1. Launch the application
2. The setup wizard will guide you through:
   - Entering your Groq API key
   - Configuring global shortcut
   - Setting transcription preferences

### Recording

1. Press the global shortcut (default: Ctrl+Space)
2. Speak your text
3. Press the shortcut again to stop recording
4. Transcription appears in the output area

### System Tray

- Right-click the tray icon to access quick actions
- Open settings, show/hide window, or quit the application

## Configuration

### Settings

Access settings via the system tray or main window:

- **API Key**: Your Groq Whisper API key
- **Global Shortcut**: Key combination to activate recording
- **Language**: Transcription language preference
- **Auto-copy**: Automatically copy transcription to clipboard

### Environment Variables

- `VITE_GROQ_API_KEY`: Groq API key (can be set via UI)

## Troubleshooting

### Global Shortcut Not Working

- Ensure you're running X11 (Wayland is not supported)
- Check if another application is using the same shortcut
- Restart the application after changing shortcut settings

### Transcription Fails

- Verify your Groq API key is valid
- Check your internet connection
- Ensure your microphone is accessible

### Build Errors

- Update system dependencies: `sudo apt update && sudo apt upgrade`
- Reinstall Rust: `rustup update`
- Clear npm cache: `npm cache clean --force`

## Development Setup

### Prerequisites

- Node.js 18+
- Rust (latest stable)
- System dependencies listed in Installation section

### Commands

```bash
# Install dependencies
npm install

# Start development server (frontend)
npm run dev

# Build frontend
npm run build

# Run Tauri app in development mode
cargo tauri dev

# Build production application
cargo tauri build
```

### Project Structure

- `src/` - Frontend React/TypeScript code
- `src-tauri/` - Rust backend code
- `public/` - Static assets

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Known Issues

- Audio module incomplete (Plans 2-3 not implemented)
- X11 only (Wayland not supported for global shortcuts)
