FROM rust:1-bookworm AS builder

# Install system dependencies (clang required by cargo-xwin)
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    wget \
    file \
    pkg-config \
    libssl-dev \
    clang \
    llvm \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 24 and pnpm
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm@9

# Add Windows MSVC target and install cargo-xwin
RUN rustup target add x86_64-pc-windows-msvc && \
    cargo install cargo-xwin --locked

WORKDIR /app

# Copy project files
COPY . .

# Install frontend dependencies and build
RUN pnpm install
RUN pnpm build

# Cross-compile Windows .exe
# TAURI_CONFIG strips devUrl so tauri-build does not set cfg(dev),
# which would cause the release binary to load from localhost instead of embedded assets.
RUN cd src-tauri && \
    TAURI_CONFIG='{"build":{"devUrl":null}}' \
    cargo xwin build --release --target x86_64-pc-windows-msvc

# Final stage — minimal image containing only the .exe for extraction
FROM debian:bookworm-slim
COPY --from=builder /app/src-tauri/target/x86_64-pc-windows-msvc/release/pdf-sanitizer.exe /pdf-sanitizer.exe

# To extract the binary:
#   docker build -t pdf-sanitizer-builder .
#   docker create --name extract pdf-sanitizer-builder
#   docker cp extract:/pdf-sanitizer.exe ./pdf-sanitizer.exe
#   docker rm extract
