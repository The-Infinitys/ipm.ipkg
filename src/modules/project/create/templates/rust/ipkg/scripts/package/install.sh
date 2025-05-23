#!/bin/bash
set -e

# インストールスクリプト
echo "Installing $IPKG_PACKAGE_NAME (version: $IPKG_PACKAGE_VERSION)"

# インストールモードに応じた処理
case "$IPKG_INSTALL_MODE" in
    "local")
        echo "Installing locally..."
        INSTALL_DIR="$HOME/.local/bin"
        CONFIG_DIR="$HOME/.config/$IPKG_PACKAGE_NAME"
        ;;
    "global")
        echo "Installing globally..."
        INSTALL_DIR="/usr/bin"
        CONFIG_DIR="/etc/$IPKG_PACKAGE_NAME"
        ;;
    *)
        echo "Unknown install mode: $IPKG_INSTALL_MODE"
        exit 1
        ;;
esac

# インストールディレクトリの作成
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"

# 実行ファイルのコピー
cp data/usr/bin/$IPKG_PACKAGE_NAME "$INSTALL_DIR/$IPKG_PACKAGE_NAME"
chmod 755 "$INSTALL_DIR/$IPKG_PACKAGE_NAME"

# 設定ファイルのコピー（存在する場合）
if [ -d "config" ]; then
    cp -r config/* "$CONFIG_DIR/"
fi

echo "Installation completed successfully" 