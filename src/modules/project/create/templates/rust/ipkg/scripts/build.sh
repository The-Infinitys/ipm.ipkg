#!/bin/bash

set -e

# Ensure project name and version are set
if [ -z "$IPKG_PROJECT_NAME" ] || [ -z "$IPKG_PROJECT_VERSION" ]; then
    echo "Error: IPKG_PROJECT_NAME and IPKG_PROJECT_VERSION must be set" >&2
    exit 1
fi

# Default to debug mode if IPKG_BUILD_MODE is unset
BUILD_MODE="${IPKG_BUILD_MODE:-debug}"

# Validate build mode
case "$BUILD_MODE" in
    release|debug)
        ;;
    *)
        echo "Error: Invalid IPKG_BUILD_MODE: $BUILD_MODE (must be 'release' or 'debug')" >&2
        exit 1
        ;;
esac

echo "Building $IPKG_PROJECT_NAME version $IPKG_PROJECT_VERSION in $BUILD_MODE mode"

# Run cargo build
if [ "$BUILD_MODE" = "release" ]; then
    cargo build --release
else
    cargo build
fi

echo "Build completed successfully"