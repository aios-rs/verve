#!/bin/bash
# Build script for Verve cross-platform packaging

set -e

 case "$1" in
    macos)
        echo "Building for macOS..."
        cargo build --release
        # TODO: Add .app and .dmg creation
        ;;
    linux)
        echo "Building for Linux..."
        cargo build --release
        # TODO: Add .deb creation
        ;;
    windows)
        echo "Building for Windows..."
        cargo build --release
        # TODO: Add .msi creation
        ;;
    *)
        echo "Usage: $0 {macos|linux|windows}"
        exit 1
        ;;
esac
