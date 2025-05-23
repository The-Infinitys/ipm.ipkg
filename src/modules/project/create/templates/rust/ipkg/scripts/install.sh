#!/bin/bash
# Install the package
if [ "$IPKG_INSTALL_MODE" = "local" ]; then
    INSTALL_DIR="$HOME/.local/bin"
elif [ "$IPKG_INSTALL_MODE" = "global" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    echo "Unknown install mode: $IPKG_INSTALL_MODE"
    exit 1
fi

mkdir -p "$INSTALL_DIR"
ipkg project build --release
# Find the binary
if [ -f "target/release/$IPKG_PROJECT_NAME" ]; then
    BINARY_PATH="target/release/$IPKG_PROJECT_NAME"
elif [ -f "target/debug/$IPKG_PROJECT_NAME" ]; then
    BINARY_PATH="target/debug/$IPKG_PROJECT_NAME"
else
    echo "Binary not found in target/release or target/debug"
    exit 1
fi

cp "$BINARY_PATH" "$INSTALL_DIR/"
chmod 755 "$INSTALL_DIR/$IPKG_PROJECT_NAME"