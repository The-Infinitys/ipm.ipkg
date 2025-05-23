#!/bin/bash
# Build the Rust project
if [ "$IPKG_BUILD_MODE" = "release" ]; then
    cargo build --release
elif [ "$IPKG_BUILD_MODE" = "debug" ]; then
    cargo build
else
    echo "Unknown build mode: $IPKG_BUILD_MODE"
    exit 1
fi