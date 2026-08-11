# PDF Sanitizer

A modern desktop application for sanitizing PDF files and removing potentially malicious content. Built with Tauri, Rust, and Svelte for a fast, lightweight, and beautiful user experience.

## Features

- **Drag & Drop UI** - Simple file management with drag-and-drop support or file selection
- **Batch Processing** - Process multiple files concurrently (configurable 1-8 concurrent files)
- **Customizable Sanitization Options**:
  - Remove metadata (default ON)
  - Remove scripts/JavaScript (default ON)
  - Remove embedded files (default ON)
  - Strip external links/URLs (default OFF)
  - Font subsetting (default OFF)
  - Image compression (default OFF)
- **File Management**:
  - Per-file progress tracking with visual progress bar
  - Stop individual files during processing
  - Automatic original PDF backup to configurable folder
  - Sanitized PDF replaces original location
- **Settings**:
  - Choose backup folder for original PDFs
  - Configure concurrent processing threads
  - All settings persisted locally

## Building Windows Executable with Docker

### Prerequisites

- Docker installed (Linux containers mode)

### Cross-compile Windows `.exe` from Linux

```bash
# Clone and enter the repository
git clone https://github.com/e-choness/pdf-sanitizer.git
cd PDFSanitizer

# Build the Docker image (compiles Windows .exe via cargo-xwin)
docker build -t pdf-sanitizer-builder .

# Extract the .exe
docker create --name extract pdf-sanitizer-builder
docker cp extract:/pdf-sanitizer.exe ./pdf-sanitizer.exe
docker rm extract
```

The resulting `pdf-sanitizer.exe` can be run on Windows without any installation.

## Project Structure

```
.
├── src/                          # Frontend (Svelte)
│   ├── App.svelte                # Main app component
│   ├── main.js                   # Entry point
│   └── components/
│       ├── FileList.svelte        # File management UI
│       ├── FileRow.svelte         # Individual file row
│       └── Settings.svelte        # Settings panel
├── src-tauri/                     # Backend (Rust)
│   ├── src/
│   │   ├── main.rs               # Tauri commands & app setup
│   │   ├── pdf_sanitizer.rs      # PDF processing logic
│   │   └── settings.rs           # Settings persistence
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Tauri configuration
├── index.html                     # HTML template
├── package.json                   # Frontend dependencies
├── vite.config.js                # Vite configuration
├── Dockerfile                     # Docker build configuration
└── docker-compose.yml            # Docker compose setup
```

## Development

### Local Setup (without Docker)

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js 24+ and pnpm
# https://nodejs.org/
npm install -g pnpm

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

## How It Works

1. **Drop or select PDF files** into the drag-drop area
2. **Configure sanitization options** in the settings panel
3. **Click "Start Converting"** to begin processing
4. **Monitor progress** for each file with the progress bar
5. **Original PDFs** are moved to your configured backup folder
6. **Sanitized PDFs** remain in the original file location

## Security Considerations

PDF processing involves parsing and rendering files which may contain exploits. The sanitizer:

- Strips metadata, scripts, and embedded files
- Re-renders PDFs to remove malicious content
- Runs entirely locally - no network communication
- Files are processed with configurable concurrency limits

For maximum security with untrusted PDFs, consider:

- Running on an isolated machine
- Processing in a sandboxed environment
- Regular security audits of the sanitization logic

## Dependencies

### Frontend

- Svelte 4.0
- Tauri API (@tauri-apps/api 2.0)
- Vite 5.4
- @tauri-apps/plugin-dialog (native file picker)

### Backend

- Tauri 2.x
- Rust 2021 edition
- Tokio (async runtime)
- Serde (serialization)
- tauri-plugin-dialog (file/folder dialogs)

## License

See LICENSE file for details.

## Credits

Original CLI implementation by Lucas Andrade Cioffi
Modern desktop UI by Beili (Echo) Yin
