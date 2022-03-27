#!/bin/bash

set -e

MACOS_APP_DIR=avtomat-dos.app
MACOS_APP_NAME=avtomat-dos
BIN_NAME=avtomat-dos

echo "Build"
cargo build --target x86_64-apple-darwin --release

echo "Setup folders"
rm -rf $MACOS_APP_DIR
mkdir -p $MACOS_APP_DIR/Contents/MacOS
mkdir -p $MACOS_APP_DIR/Contents/Resources

echo "Copy files"
cp target/x86_64-apple-darwin/release/$BIN_NAME $MACOS_APP_DIR/Contents/MacOS/$BIN_NAME
cp assets/AppIcon.icns $MACOS_APP_DIR/Contents/Resources
cp assets/Info.plist $MACOS_APP_DIR/Contents

echo "Signing"
sudo codesign -fs Grrwahrr $MACOS_APP_DIR

echo "Create dmg"
mkdir $MACOS_APP_NAME
mv $MACOS_APP_DIR $MACOS_APP_NAME
ln -s /Applications $MACOS_APP_NAME/Applications
rm -rf $MACOS_APP_NAME/.Trashes
hdiutil create $MACOS_APP_NAME.macOS.intel.x64.dmg -srcfolder $MACOS_APP_NAME -ov
rm -rf $MACOS_APP_NAME