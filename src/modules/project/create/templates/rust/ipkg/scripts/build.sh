#!/bin/bash
set -e

# ビルドスクリプト
echo "Building $IPKG_PACKAGE_NAME"

# ビルド処理
case "$IPKG_PACKAGE_TARGET" in
    "source-build")
        echo "Building from source..."
        cargo build --release
        ;;
    "normal")
        echo "Building normal package..."
        cargo build --release
        ;;
    "minimal")
        echo "Building minimal package..."
        cargo build --release --no-default-features
        ;;
    *)
        echo "Unknown build target: $IPKG_PACKAGE_TARGET"
        exit 1
        ;;
esac

# ビルドディレクトリの作成
mkdir -p usr/bin

# ビルド成果物のコピー
cp target/release/$IPKG_PACKAGE_NAME usr/bin/$IPKG_PACKAGE_NAME
chmod 755 usr/bin/$IPKG_PACKAGE_NAME

echo "Build completed successfully"
