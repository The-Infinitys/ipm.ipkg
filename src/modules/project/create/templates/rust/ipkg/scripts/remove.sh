#!/bin/bash
# Remove the package
if [ "$IPKG_REMOVE_MODE" = "local" ]; then
    INSTALL_DIR="$HOME/.local/bin"
elif [ "$IPKG_REMOVE_MODE" = "global" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    echo "Unknown remove mode: $IPKG_REMOVE_MODE"
    exit 1
fi

rm -f "$INSTALL_DIR/$IPKG_PROJECT_NAME"