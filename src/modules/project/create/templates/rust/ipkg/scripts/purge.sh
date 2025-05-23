#!/bin/bash
# Purge the package
if [ "$IPKG_PURGE_MODE" = "local" ]; then
    INSTALL_DIR="$HOME/.local"
elif [ "$IPKG_PURGE_MODE" = "global" ]; then
    INSTALL_DIR="/usr/local"
else
    echo "Unknown purge mode: $IPKG_PURGE_MODE"
    exit 1
fi

rm -f "$INSTALL_DIR/bin/$IPKG_PROJECT_NAME"
# Remove configuration if exists
rm -rf "$INSTALL_DIR/etc/$IPKG_PROJECT_NAME"