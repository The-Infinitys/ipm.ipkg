#!/bin/bash
set -e

# クリーンアップスクリプト
echo "Cleaning up $IPKG_PACKAGE_NAME"

# 一時ファイルの削除
rm -rf ./data
rm -rf ./usr/bin

# Cargoのビルド成果物の削除
cargo clean

# その他の一時ファイルの削除
if [ -d "target" ]; then
    rm -rf target
fi

# ログファイルの削除
if [ -f "Cargo.lock" ]; then
    rm Cargo.lock
fi

echo "Cleanup completed successfully" 