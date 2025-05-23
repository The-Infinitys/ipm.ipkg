#!/bin/bash
set -e

# パッケージングのメインスクリプト
echo "Starting package process for $IPKG_PACKAGE_NAME (version: $IPKG_PACKAGE_VERSION)"

# パッケージング用の一時ディレクトリを作成
rm -rf ./data
mkdir -p data/usr/bin

# パッケージング
case "$IPKG_PACKAGE_TARGET" in
    "source-build")
        echo "Building from source..."
        # ソースビルド用の処理をここに追加
        cp -a ./usr/bin/$IPKG_PACKAGE_NAME ./data/usr/bin/$IPKG_PACKAGE_NAME
        ;;
    "normal")
        echo "Creating normal package..."
        cp -a ./usr/bin/$IPKG_PACKAGE_NAME ./data/usr/bin/$IPKG_PACKAGE_NAME
        ;;
    "minimal")
        echo "Creating minimal package..."
        cp -a ./usr/bin/$IPKG_PACKAGE_NAME ./data/usr/bin/$IPKG_PACKAGE_NAME
        ;;
    *)
        echo "Unknown package target: $IPKG_PACKAGE_TARGET"
        exit 1
        ;;
esac

# 実行権限の設定
chmod 755 ./data/usr/bin/$IPKG_PACKAGE_NAME

echo "Package process completed successfully"