# Quick Start Guide

## Fastest Path: Docker Setup (No Dependencies Needed)

### 1. Install Docker & Docker Compose
- **Windows:** [Docker Desktop](https://www.docker.com/products/docker-desktop)
- **Mac:** [Docker Desktop](https://www.docker.com/products/docker-desktop)
- **Linux:** `sudo apt install docker.io docker-compose` (or equivalent for your distro)

### 2. Clone & Run

```bash
git clone <repo-url>
cd PDFSanitizer
docker-compose up
```

That's it! The application will:
- Download all dependencies automatically
- Build in a container
- Start the dev server

Access the app at: **http://localhost:5173**

---

## Local Setup (For Active Development)

### Prerequisites
- **Rust 1.70+** → [Install](https://rustup.rs/)
- **Node.js 18+** → [Install](https://nodejs.org/)
- **pnpm** → `npm install -g pnpm`

### Setup Steps

```bash
# 1. Clone repository
git clone <repo-url>
cd PDFSanitizer

# 2. Install dependencies
pnpm install

# 3. Run development server
cargo tauri dev
```

A window will open with the application running!

---

## Building for Release

### With Docker
```bash
docker-compose run --rm pdf-sanitizer cargo tauri build
```

Find the executable in: `src-tauri/target/release/`

### Local Build
```bash
cargo tauri build
```

---

## First Time Users

### Adding Files
1. **Drag & Drop:** Grab PDF files and drop them onto the app window
2. **Select Folder:** Click "Select Folder" button to browse

### Processing
1. **Choose Options:** Toggle sanitization options in the right panel
2. **Set Backup Folder:** Choose where to store original PDFs (Settings → Output Folder)
3. **Start:** Click "Start Converting" when ready
4. **Monitor:** Watch progress bars update for each file
5. **Done:** Remove completed files from list with the ✕ button

### Default Sanitization Options
- ✅ Remove Metadata - Strips author, dates, properties
- ✅ Remove Scripts - Disables JavaScript
- ✅ Remove Embedded Files - Removes attachments
- ☐ Others - Disabled by default (toggle as needed)

---

## Troubleshooting

### Docker
**Issue:** Port 5173 already in use
```bash
# Kill existing process or use different port
docker-compose run -p 5174:5173 pdf-sanitizer
```

**Issue:** Permission denied on Linux
```bash
sudo usermod -aG docker $USER
```

### Local Build
**Issue:** "failed to find Rust toolchain"
```bash
rustup update
```

**Issue:** "pnpm: command not found"
```bash
npm install -g pnpm
```

---

## Useful Commands

### Development
```bash
# Watch file changes and hot-reload
cargo tauri dev

# Format code
cargo fmt

# Check for issues
cargo clippy
```

### Production Build
```bash
# Create optimized executable
cargo tauri build

# Find output in: src-tauri/target/release/pdf-sanitizer
```

---

## What's Happening Behind the Scenes?

1. **Files Added** → Stored in app state
2. **Start Converting** → Backend spawns sanitization tasks
3. **Concurrent Processing** → Multiple files handled simultaneously (1-8 threads)
4. **Sanitization Pipeline:**
   - Read PDF from disk
   - Remove metadata/scripts/files based on settings
   - Write sanitized PDF to original location
   - Move original to backup folder
5. **Events Sent** → Frontend updates UI with progress/completion
6. **Complete** → File stays in list until manually removed

---

## Testing with Sample PDFs

Create a test PDF to try:
```bash
# Linux/Mac
echo "Sample text" | convert -density 150 -colorspace RGB label:@ test.pdf

# Or download from: https://github.com/mozilla/pdf.js/tree/master/test/pdfs
```

---

## Performance Tips

- **Concurrent Files:** Adjust in Settings (higher = faster but more CPU)
- **File Size:** Larger PDFs take longer to process
- **Backup Folder:** Use SSD for faster file moves
- **Compression:** Disable unused options for better performance

---

## Next Steps

- See [DEVELOPMENT.md](./DEVELOPMENT.md) for architecture details
- Check [README.md](./README.md) for full documentation
- Report issues with steps to reproduce

Happy sanitizing! 🎉
