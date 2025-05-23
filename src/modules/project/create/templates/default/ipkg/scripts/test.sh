#!/bin/bash
set -e

# テストスクリプト
echo "Testing $IPKG_PACKAGE_NAME"

# テストディレクトリの確認
if [ ! -d "tests" ]; then
    echo "No tests directory found"
    exit 0
fi

# テストの実行
TEST_COUNT=0
PASS_COUNT=0
FAIL_COUNT=0

for test_file in tests/*.sh; do
    if [ -f "$test_file" ]; then
        echo "Running test: $test_file"
        TEST_COUNT=$((TEST_COUNT + 1))
        
        if bash "$test_file"; then
            echo "Test passed: $test_file"
            PASS_COUNT=$((PASS_COUNT + 1))
        else
            echo "Test failed: $test_file"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
    fi
done

# テスト結果の表示
echo "Test Summary:"
echo "Total tests: $TEST_COUNT"
echo "Passed: $PASS_COUNT"
echo "Failed: $FAIL_COUNT"

if [ $FAIL_COUNT -gt 0 ]; then
    echo "Tests completed with failures"
    exit 1
else
    echo "Tests completed successfully"
    exit 0
fi 