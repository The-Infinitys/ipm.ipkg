#!/bin/bash
# Install the package on the target system

# Determine installation directory
if [ "$IPKG_INSTALL_MODE" = "global" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ "$IPKG_INSTALL_MODE" = "local" ]; then
    INSTALL_DIR="$HOME/.local/bin"
else
    echo "Unknown install mode: $IPKG_INSTALL_MODE"
    exit 1
fi

# Create installation directory
mkdir -p "$INSTALL_DIR"

# Copy the binary
if [ -f "./$IPKG_PROJECT_NAME" ]; then
    cp "./$IPKG_PROJECT_NAME" "$INSTALL_DIR/"
else
    echo "Binary $IPKG_PROJECT_NAME not found in current directory"
    exit 1
fi

# Set execute permissions
chmod 755 "$INSTALL_DIR/$IPKG_PROJECT_NAME"

# Create a symlink for global installation
if [ "$IPKG_INSTALL_MODE" = "global" ] && [ ! -e /usr/bin/$IPKG_PROJECT_NAME ]; then
    ln -s "$INSTALL_DIR/$IPKG_PROJECT_NAME" /usr/bin/$IPKG_PROJECT_NAME
fi

echo "Installation completed for $IPKG_PROJECT_NAME"