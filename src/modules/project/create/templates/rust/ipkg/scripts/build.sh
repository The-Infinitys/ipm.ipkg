if [ $IPKG_BUILD_MODE = "debug" ]; then
    cargo build
elif [$IPKG_BUILD_MODE = "release" ]; then
    cargo build --release
fi
