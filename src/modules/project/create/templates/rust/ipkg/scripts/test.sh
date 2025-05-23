#!/bin/bash
set -e

# テストスクリプト
echo "Testing $IPKG_PACKAGE_NAME"

# Cargoテストの実行
cargo test

echo "Tests completed successfully" 