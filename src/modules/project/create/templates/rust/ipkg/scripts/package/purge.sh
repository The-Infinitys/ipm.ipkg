#!/bin/bash
# Purge the package from the target system

# Determine installation directory
if [ "$IPKG_PURGE_MODE" = "global" ]; then
    INSTALL_DIR="/usr/local/bin"
    CONFIG_DIR="/etc/$IPKG_PROJECT_NAME"
elif [ "$IPKG_PURGE_MODE" = "local" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.config/$IPKG_PROJECT_NAME"
else
    echo "Unknown purge mode: $IPKG_PURGE_MODE"
    exit 1
fi

# Remove the binary
if [ -f "$INSTALL_DIR/$IPKG_PROJECT_NAME" ]; then
    rm -f "$INSTALL_DIR/$IPKG_PROJECT_NAME"
else
    echo "Binary $IPKG_PROJECT_NAME not found in $INSTALL_DIR"
fi

# Remove symlink for global installation
if [ "$IPKG_PURGE_MODE" = "global" ] && [ -L /usr/bin/$IPKG_PROJECT_NAME ]; then
    rm -f /usr/bin/$IPKG_PROJECT_NAME
fi

# Remove configuration files if they exist
if [ -d "$CONFIG_DIR" ]; then
    rm -rf "$CONFIG_DIR"
fi

echo "Purge completed for $IPKG_PROJECT_NAME"