# Dockerfile for MASH-installer
# Multi-stage build for creating a containerized installer

# Stage 1: Build the Rust application
FROM rust:1.93 as builder

WORKDIR /app

# Copy workspace manifest and all crate manifests first (dependency cache layer)
COPY Cargo.toml Cargo.lock ./
COPY installer-cli/Cargo.toml installer-cli/
COPY installer-core/Cargo.toml installer-core/
COPY installer-arch/Cargo.toml installer-arch/
COPY installer-debian/Cargo.toml installer-debian/
COPY installer-fedora/Cargo.toml installer-fedora/
COPY wallpaper-downloader/Cargo.toml wallpaper-downloader/

# Create placeholder source files so cargo can parse manifests without source trees.
# These are replaced by the real COPY steps below.
RUN mkdir -p installer-cli/src installer-core/src \
             installer-arch/src installer-debian/src \
             installer-fedora/src wallpaper-downloader/src \
    && echo 'fn main() {}' > installer-cli/src/main.rs \
    && touch installer-core/src/lib.rs \
             installer-arch/src/lib.rs \
             installer-debian/src/lib.rs \
             installer-fedora/src/lib.rs \
             wallpaper-downloader/src/lib.rs

# Fetch (and implicitly cache) all dependencies
RUN cargo fetch --target x86_64-unknown-linux-gnu

# Copy real source files (overwrites placeholders)
COPY installer-cli/src/ installer-cli/src/
COPY installer-core/src/ installer-core/src/
COPY installer-arch/src/ installer-arch/src/
COPY installer-debian/src/ installer-debian/src/
COPY installer-fedora/src/ installer-fedora/src/
COPY wallpaper-downloader/src/ wallpaper-downloader/src/
COPY resources/ resources/

# Build release binary
RUN cargo build --release --bin mash-setup

# Stage 2: Create runtime image
FROM debian:bookworm-slim as runtime

RUN apt-get update && apt-get install -y \
    bash \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /root/

# Copy the binary from builder
COPY --from=builder /app/target/release/mash-setup /usr/local/bin/mash-setup

# Copy install script
COPY install.sh /usr/local/bin/install-mash

# Make install script executable
RUN chmod +x /usr/local/bin/install-mash

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/mash-setup"]

# Default command (can be overridden)
CMD ["--help"]
