#!/bin/bash
# Remove the package from the target system

# Determine installation directory
if [ "$IPKG_REMOVE_MODE" = "global" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ "$IPKG_REMOVE_MODE" = "local" ]; then
    INSTALL_DIR="$HOME/.local/bin"
else
    echo "Unknown remove mode: $IPKG_REMOVE_MODE"
    exit 1
fi

# Remove the binary
if [ -f "$INSTALL_DIR/$IPKG_PROJECT_NAME" ]; then
    rm -f "$INSTALL_DIR/$IPKG_PROJECT_NAME"
else
    echo "Binary $IPKG_PROJECT_NAME not found in $INSTALL_DIR"
fi

# Remove symlink for global installation
if [ "$IPKG_REMOVE_MODE" = "global" ] && [ -L /usr/bin/$IPKG_PROJECT_NAME ]; then
    rm -f /usr/bin/$IPKG_PROJECT_NAME
fi

echo "Removal completed for $IPKG_PROJECT_NAME"