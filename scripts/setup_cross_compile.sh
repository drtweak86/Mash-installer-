#!/bin/bash
# Setup script for cross-compilation - ensures SQLite3 is available for target architecture

set -e

echo "Setting up cross-compilation environment..."

# Install cross-compilation tools and dependencies
if [ -f /etc/debian_version ] || [ -f /etc/ubuntu-version ]; then
    echo "Debian/Ubuntu system detected"
    
    # Install basic build tools
    sudo apt-get update
    sudo apt-get install -y \
        build-essential \
        pkg-config \
        libssl-dev
    
    # Install SQLite3 development files
    sudo apt-get install -y libsqlite3-dev
    
    # For aarch64 cross-compilation specifically
    sudo apt-get install -y \
        gcc-aarch64-linux-gnu \
        g++-aarch64-linux-gnu \
        libc6-dev-arm64-cross
    
    # Create symlinks for cross-compilation
    sudo mkdir -p /usr/aarch64-linux-gnu/lib
    sudo ln -sf /usr/lib/aarch64-linux-gnu/libsqlite3.so.0 /usr/aarch64-linux-gnu/lib/libsqlite3.so 2>/dev/null || true
    
elif [ -f /etc/redhat-release ]; then
    echo "RedHat/Fedora system detected"
    sudo dnf install -y \
        gcc \
        gcc-toolset-12-aarch64-runtime \
        sqlite-devel \
        openssl-devel
else
    echo "Unsupported system for cross-compilation setup"
    exit 1
fi

echo "Cross-compilation environment setup complete"
