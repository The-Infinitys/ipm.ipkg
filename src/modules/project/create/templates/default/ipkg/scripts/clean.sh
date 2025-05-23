#!/bin/bash
set -e

# クリーンアップスクリプト
echo "Cleaning up $IPKG_PACKAGE_NAME"

# 一時ファイルの削除
rm -rf ./data
rm -rf ./usr/bin

# ビルド成果物の削除
if [ -d "build" ]; then
    rm -rf build
fi

echo "Cleanup completed successfully" 