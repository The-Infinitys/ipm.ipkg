mkdir -p ./build/$IPKG_BUILD_MODE
rm -rf ./build/$IPKG_BUILD_MODE/*
cp src/main.sh ./build/$IPKG_BUILD_MODE/$IPKG_PACKAGE_NAME
