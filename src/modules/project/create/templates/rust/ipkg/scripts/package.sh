#!/bin/bash
set -e

# パッケージングのメインスクリプト
echo "Starting package process for $IPKG_PACKAGE_NAME (version: $IPKG_PACKAGE_VERSION)"

# パッケージング用の一時ディレクトリを作成
rm -rf ./data
mkdir -p data/usr/bin

# ビルドとパッケージング
case "$IPKG_PACKAGE_TARGET" in
    "source-build")
        echo "Building from source..."
        cargo build --release
        cp -a target/release/$IPKG_PACKAGE_NAME ./data/usr/bin/$IPKG_PACKAGE_NAME
        ;;
    "normal")
        echo "Creating normal package..."
        cargo build --release
        cp -a target/release/$IPKG_PACKAGE_NAME ./data/usr/bin/$IPKG_PACKAGE_NAME
        ;;
    "minimal")
        echo "Creating minimal package..."
        cargo build --release --no-default-features
        cp -a target/release/$IPKG_PACKAGE_NAME ./data/usr/bin/$IPKG_PACKAGE_NAME
        ;;
    *)
        echo "Unknown package target: $IPKG_PACKAGE_TARGET"
        exit 1
        ;;
esac

# 実行権限の設定
chmod 755 ./data/usr/bin/$IPKG_PACKAGE_NAME

echo "Package process completed successfully" 