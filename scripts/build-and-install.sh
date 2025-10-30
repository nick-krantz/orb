#!/usr/bin/env bash
set -euo pipefail

BINARY_NAME="orb"

echo "Building $BINARY_NAME..."
cargo build --release

SOURCE="target/release/$BINARY_NAME"
DEST="/Users/nickkrantz/.local/bin/$BINARY_NAME"

echo "Installing $BINARY_NAME to $DEST..."
cp "$SOURCE" "$DEST"

chmod +x "$DEST"

echo "Installed successfully!"
echo
echo "You can now run it with: $BINARY_NAME"
echo "(Make sure ~/usr/local/bin/ is in your PATH)"
