#!/bin/bash
# Prepare the package for output (e.g., .ipkg file)

# Build the project if not already built
if [ "$IPKG_BUILD_MODE" = "release" ]; then
    cargo build --release
elif [ "$IPKG_BUILD_MODE" = "debug" ]; then
    cargo build
else
    echo "Unknown build mode: $IPKG_BUILD_MODE"
    exit 1
fi

# Create package directory
mkdir -p package/

# Copy the binary
if [ -f "target/release/$IPKG_PROJECT_NAME" ]; then
    cp "target/release/$IPKG_PROJECT_NAME" "package/"
elif [ -f "target/debug/$IPKG_PROJECT_NAME" ]; then
    cp "target/debug/$IPKG_PROJECT_NAME" "package/"
else
    echo "Binary not found in target/release or target/debug"
    exit 1
fi

# Copy package.yaml
cp package.yaml package/

# Ensure package scripts are copied
if [ -f "ipkg/scripts/package/install.sh" ]; then
    cp ipkg/scripts/package/install.sh package/
fi
if [ -f "ipkg/scripts/package/remove.sh" ]; then
    cp ipkg/scripts/package/remove.sh package/
fi
if [ -f "ipkg/scripts/package/purge.sh" ]; then
    cp ipkg/scripts/package/purge.sh package/
fi

echo "Package preparation complete. Files are in package/"