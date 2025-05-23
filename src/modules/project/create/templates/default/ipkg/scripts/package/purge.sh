#!/bin/bash
set -e

# 完全削除スクリプト
echo "Purging $IPKG_PACKAGE_NAME"

# インストールモードに応じた処理
case "$IPKG_INSTALL_MODE" in
    "local")
        echo "Purging from local installation..."
        INSTALL_DIR="$HOME/.local/bin"
        CONFIG_DIR="$HOME/.config/$IPKG_PACKAGE_NAME"
        CACHE_DIR="$HOME/.cache/$IPKG_PACKAGE_NAME"
        ;;
    "global")
        echo "Purging from global installation..."
        INSTALL_DIR="/usr/bin"
        CONFIG_DIR="/etc/$IPKG_PACKAGE_NAME"
        CACHE_DIR="/var/cache/$IPKG_PACKAGE_NAME"
        ;;
    *)
        echo "Unknown install mode: $IPKG_INSTALL_MODE"
        exit 1
        ;;
esac

# 実行ファイルの削除
if [ -f "$INSTALL_DIR/$IPKG_PACKAGE_NAME" ]; then
    rm "$INSTALL_DIR/$IPKG_PACKAGE_NAME"
    echo "Executable removed"
fi

# 設定ファイルの削除
if [ -d "$CONFIG_DIR" ]; then
    rm -rf "$CONFIG_DIR"
    echo "Configuration files removed"
fi

# キャッシュファイルの削除
if [ -d "$CACHE_DIR" ]; then
    rm -rf "$CACHE_DIR"
    echo "Cache files removed"
fi

echo "Package purged successfully"
