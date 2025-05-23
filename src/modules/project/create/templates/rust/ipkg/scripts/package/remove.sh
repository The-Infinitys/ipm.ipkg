#!/bin/bash
set -e

# アンインストールスクリプト
echo "Removing $IPKG_PACKAGE_NAME"

# インストールモードに応じた処理
case "$IPKG_INSTALL_MODE" in
    "local")
        echo "Removing from local installation..."
        INSTALL_DIR="$HOME/.local/bin"
        CONFIG_DIR="$HOME/.config/$IPKG_PACKAGE_NAME"
        ;;
    "global")
        echo "Removing from global installation..."
        INSTALL_DIR="/usr/bin"
        CONFIG_DIR="/etc/$IPKG_PACKAGE_NAME"
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

# 設定ファイルの削除（purgeではないため、設定ファイルは残す）
echo "Package removed successfully" 