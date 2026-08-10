FROM rust:latest as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    wget \
    file \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js (v20)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm

WORKDIR /app

# Copy project files
COPY . .

# Install frontend dependencies
RUN pnpm install

# Build frontend
RUN pnpm build

# Build backend (release)
RUN cd src-tauri && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy built application from builder
COPY --from=builder /app/src-tauri/target/release/pdf-sanitizer /usr/local/bin/

WORKDIR /app

# Expose port for webview
EXPOSE 8080

# Run the application
CMD ["pdf-sanitizer"]
