#!/bin/bash

set -e

# Ensure project name, version, and remove mode are set
if [ -z "$IPKG_PROJECT_NAME" ] || [ -z "$IPKG_PROJECT_VERSION" ]; then
    echo "Error: IPKG_PROJECT_NAME and IPKG_PROJECT_VERSION must be set" >&2
    exit 1
fi
if [ -z "$IPKG_REMOVE_MODE" ]; then
    echo "Error: IPKG_REMOVE_MODE must be set (local or global)" >&2
    exit 1
fi

# Validate remove mode
case "$IPKG_REMOVE_MODE" in
    local|global)
        ;;
    *)
        echo "Error: Invalid IPKG_REMOVE_MODE: $IPKG_REMOVE_MODE (must be 'local' or 'global')" >&2
        exit 1
        ;;
esac

# Determine binary path
if [ "$IPKG_REMOVE_MODE" = "local" ]; then
    BINARY_PATH="$HOME/.ipkg/bin/$IPKG_PROJECT_NAME"
else
    BINARY_PATH="/usr/local/bin/$IPKG_PROJECT_NAME"
fi

echo "Removing $IPKG_PROJECT_NAME version $IPKG_PROJECT_VERSION from $BINARY_PATH"

# Remove binary if it exists
if [ -f "$BINARY_PATH" ]; then
    rm "$BINARY_PATH"
    echo "Removed successfully"
else
    echo "Warning: Binary '$BINARY_PATH' not found, nothing to remove"
fi