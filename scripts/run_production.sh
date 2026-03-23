#!/usr/bin/env bash

set -euo pipefail

WORKSPACE_DIR="/home/nikola-bozin/Documents/projects2.0/mordexel/core"
BINARY_PATH="$WORKSPACE_DIR/target/release/engine"
SESSION_FILE="$WORKSPACE_DIR/mordexel.session"

echo "Moving to workspace directory..."
cd "$WORKSPACE_DIR"

if ! command -v git >/dev/null 2>&1; then
    echo "git is not installed or not available in PATH"
    exit 1
fi

BUILD_VERSION="$(git rev-parse --short HEAD)"

if [ -z "$BUILD_VERSION" ]; then
    echo "Failed to determine git hash for BUILD_VERSION"
    exit 1
fi

echo "BUILD_VERSION=$BUILD_VERSION"

echo "Building project in release mode..."
BUILD_VERSION="$BUILD_VERSION" cargo build --release

if [ ! -x "$BINARY_PATH" ]; then
    echo "Binary not found or not executable at $BINARY_PATH"
    exit 1
fi

if [ ! -e "$SESSION_FILE" ]; then
    echo "Session file not found at $SESSION_FILE"
    exit 1
fi

echo "Running engine from: $(pwd)"
echo "Using session file: $SESSION_FILE"

exec "$BINARY_PATH"