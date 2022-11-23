#!/bin/bash

##################################################
# We call this from an Xcode run script.
# From: https://chinedufn.github.io/swift-bridge/building/xcode-and-cargo/index.html
##################################################

set -e

if [[ -z "$PROJECT_DIR" ]]; then
    echo "Must provide PROJECT_DIR environment variable set to the Xcode project directory." 1>&2
    exit 1
fi

cd $PROJECT_DIR

export PATH="$HOME/.cargo/bin:$PATH"

export SWIFT_BRIDGE_OUT_DIR="${PROJECT_DIR}/Generated"

# Without this we can't compile on MacOS Big Sur
# https://github.com/TimNN/cargo-lipo/issues/41#issuecomment-774793892
if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]; then
  export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

# if [ $ENABLE_PREVIEWS == "NO" ]; then

  if [[ $CONFIGURATION == "Release" ]]; then
      echo "BUIlDING FOR RELEASE"
      
      cargo lipo --release --manifest-path ../spotify-ffi/Cargo.toml --targets aarch64-apple-darwin #,x86_64-apple-darwin
  else
      echo "BUIlDING FOR DEBUG"

      cargo lipo --manifest-path ../spotify-ffi/Cargo.toml --targets aarch64-apple-darwin #,x86_64-apple-darwin
  fi
  
# else
#   echo "Skipping the script because of preview mode"
# fi
