#!/bin/bash
set -e

# ビルドスクリプト
echo "Building $IPKG_PACKAGE_NAME"

# ビルドディレクトリの作成
mkdir -p usr/bin

# ビルド処理
case "$IPKG_PACKAGE_TARGET" in
    "source-build")
        echo "Building from source..."
        if [ -f "src/main.sh" ]; then
            cp src/main.sh usr/bin/$IPKG_PACKAGE_NAME
        else
            echo "Error: Source file not found"
            exit 1
        fi
        ;;
    "normal")
        echo "Building normal package..."
        if [ -f "src/main.sh" ]; then
            cp src/main.sh usr/bin/$IPKG_PACKAGE_NAME
        else
            echo "Error: Source file not found"
            exit 1
        fi
        ;;
    "minimal")
        echo "Building minimal package..."
        if [ -f "src/main.sh" ]; then
            cp src/main.sh usr/bin/$IPKG_PACKAGE_NAME
        else
            echo "Error: Source file not found"
            exit 1
        fi
        ;;
    *)
        echo "Unknown build target: $IPKG_PACKAGE_TARGET"
        exit 1
        ;;
esac

# 実行権限の設定
chmod 755 usr/bin/$IPKG_PACKAGE_NAME

echo "Build completed successfully"
