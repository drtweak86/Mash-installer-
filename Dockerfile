# Dockerfile for MASH-installer
# Multi-stage build for creating a containerized installer

# Stage 1: Build the Rust application
FROM rust:1.93 as builder

WORKDIR /app

# Copy workspace manifest and all crate manifests first (dependency cache layer)
COPY Cargo.toml Cargo.lock ./
COPY installer-cli/Cargo.toml installer-cli/
COPY installer-core/Cargo.toml installer-core/
COPY installer-drivers/Cargo.toml installer-drivers/
COPY installer-model/Cargo.toml installer-model/
COPY mash-system/Cargo.toml mash-system/
COPY mash-wallpaper/Cargo.toml mash-wallpaper/
COPY wallpaper-downloader/Cargo.toml wallpaper-downloader/
COPY xtask/Cargo.toml xtask/
COPY workspace-hack/Cargo.toml workspace-hack/

# Create placeholder source files so cargo can parse manifests without source trees.
RUN mkdir -p installer-cli/src installer-core/src \
             installer-drivers/src installer-model/src \
             mash-system/src mash-wallpaper/src \
             wallpaper-downloader/src \
             xtask/src workspace-hack/src \
    && echo 'fn main() {}' > installer-cli/src/main.rs \
    && echo 'fn main() {}' > xtask/src/main.rs \
    && touch installer-core/src/lib.rs \
             installer-drivers/src/lib.rs \
             installer-model/src/lib.rs \
             mash-system/src/lib.rs \
             mash-wallpaper/src/lib.rs \
             wallpaper-downloader/src/lib.rs \
             workspace-hack/src/lib.rs

# Fetch dependencies
RUN cargo fetch --target x86_64-unknown-linux-gnu

# Copy real source files
COPY installer-cli/src/ installer-cli/src/
COPY installer-core/src/ installer-core/src/
COPY installer-drivers/src/ installer-drivers/src/
COPY installer-model/src/ installer-model/src/
COPY mash-system/src/ mash-system/src/
COPY mash-wallpaper/src/ mash-wallpaper/src/
COPY wallpaper-downloader/src/ wallpaper-downloader/src/
COPY resources/ resources/

# Build release binary
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev libsqlite3-dev \
    && cargo build --release --bin mash-setup

# Stage 2: Create runtime image
FROM debian:bookworm-slim as runtime

RUN apt-get update && apt-get install -y \
    bash \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /root/

# Copy binary from builder (handling custom target path if present)
COPY --from=builder /app/target/release/mash-setup /usr/local/bin/mash-setup

# Copy install script
COPY install.sh /usr/local/bin/install-mash
RUN chmod +x /usr/local/bin/install-mash

ENTRYPOINT ["/usr/local/bin/mash-setup"]
CMD ["--help"]
