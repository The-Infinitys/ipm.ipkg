#!/bin/bash

set -e

# Ensure project name, version, and install mode are set
if [ -z "$IPKG_PROJECT_NAME" ] || [ -z "$IPKG_PROJECT_VERSION" ]; then
    echo "Error: IPKG_PROJECT_NAME and IPKG_PROJECT_VERSION must be set" >&2
    exit 1
fi
if [ -z "$IPKG_INSTALL_MODE" ]; then
    echo "Error: IPKG_INSTALL_MODE must be set (local or global)" >&2
    exit 1
fi

# Default to debug mode if IPKG_BUILD_MODE is unset
BUILD_MODE="${IPKG_BUILD_MODE:-debug}"

# Validate modes
case "$IPKG_INSTALL_MODE" in
    local|global)
        ;;
    *)
        echo "Error: Invalid IPKG_INSTALL_MODE: $IPKG_INSTALL_MODE (must be 'local' or 'global')" >&2
        exit 1
        ;;
esac
case "$BUILD_MODE" in
    release|debug)
        ;;
    *)
        echo "Error: Invalid IPKG_BUILD_MODE: $BUILD_MODE (must be 'release' or 'debug')" >&2
        exit 1
        ;;
esac

# Determine source and destination paths
BINARY_DIR="target/$BUILD_MODE"
BINARY_PATH="$BINARY_DIR/$IPKG_PROJECT_NAME"
if [ "$IPKG_INSTALL_MODE" = "local" ]; then
    INSTALL_DIR="$HOME/.ipkg/bin"
else
    INSTALL_DIR="/usr/local/bin"
fi
DEST_PATH="$INSTALL_DIR/$IPKG_PROJECT_NAME"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary '$BINARY_PATH' not found. Run build first." >&2
    exit 1
fi

# Create install directory
mkdir -p "$INSTALL_DIR"

echo "Installing $IPKG_PROJECT_NAME version $IPKG_PROJECT_VERSION to $DEST_PATH"

# Copy binary
cp "$BINARY_PATH" "$DEST_PATH"

# Set executable permissions
chmod +x "$DEST_PATH"

echo "Installation completed successfully"