#/bin/bash

set -e

TARGET="${TARGET:-x86_64-apple-ios}"
APP_NAME="$(cat Cargo.toml | dasel -r toml '.package.name')"
BUNDLE_ID="$(cat Cargo.toml | dasel -r toml '.package.metadata.bundle.identifier')"

BUNDLE_CMD="cargo bundle --target $TARGET"

echo "Bundling app for iOS"
which cargo-bundle || cargo install cargo-bundle
$BUNDLE_CMD
xcrun simctl boot "iPhone 12 mini" || true 
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
xcrun simctl install booted "target/$TARGET/debug/bundle/ios/$APP_NAME.app"
xcrun simctl launch --console booted "$BUNDLE_ID"

