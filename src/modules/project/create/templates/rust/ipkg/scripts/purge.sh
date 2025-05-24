#!/bin/bash

set -e

# Ensure project name, version, and purge mode are set
if [ -z "$IPKG_PROJECT_NAME" ] || [ -z "$IPKG_PROJECT_VERSION" ]; then
    echo "Error: IPKG_PROJECT_NAME and IPKG_PROJECT_VERSION must be set" >&2
    exit 1
fi
if [ -z "$IPKG_PURGE_MODE" ]; then
    echo "Error: IPKG_PURGE_MODE must be set (local or global)" >&2
    exit 1
fi

# Validate purge mode
case "$IPKG_PURGE_MODE" in
    local|global)
        ;;
    *)
        echo "Error: Invalid IPKG_PURGE_MODE: $IPKG_PURGE_MODE (must be 'local' or 'global')" >&2
        exit 1
        ;;
esac

# Determine binary path
if [ "$IPKG_PURGE_MODE" = "local" ]; then
    BINARY_PATH="$HOME/.ipkg/bin/$IPKG_PROJECT_NAME"
else
    BINARY_PATH="/usr/local/bin/$IPKG_PROJECT_NAME"
fi

echo "Purging $IPKG_PROJECT_NAME version $IPKG_PROJECT_VERSION"

# Remove binary if it exists
if [ -f "$BINARY_PATH" ]; then
    rm "$BINARY_PATH"
    echo "Removed binary: $BINARY_PATH"
else
    echo "Warning: Binary '$BINARY_PATH' not found"
fi

# For local mode, also remove target/ directory
if [ "$IPKG_PURGE_MODE" = "local" ]; then
    TARGET_DIR="target"
    if [ -d "$TARGET_DIR" ]; then
        rm -rf "$TARGET_DIR"
        echo "Purged target directory: $TARGET_DIR"
    else
        echo "Warning: Target directory '$TARGET_DIR' not found"
    fi
fi

echo "Purge completed successfully"