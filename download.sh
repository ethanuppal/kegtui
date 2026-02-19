#!/bin/sh

# Obtains official wrappers and engines from Kegworks public sources

ARTIFACTS_FOLDER="/tmp/kegworks_download"
WRAPPER_FOLDER="$HOME/Library/Application Support/Sikarugir/Wrapper"
ENGINE_FOLDER="$HOME/Library/Application Support/Sikarugir/Engines"

WRAPPER_DOWNLOAD_NAME="Wineskin-3.1.6.tar.xz"
WRAPPER_DOWNLOAD_PATH="$ARTIFACTS_FOLDER/$WRAPPER_DOWNLOAD_NAME"
WRAPPER_URL="https://github.com/Sikarugir-App/Wrapper/releases/download/v1.0/$WRAPPER_DOWNLOAD_NAME"

ENGINE_DOWNLOAD_NAME="WS12WineSikarugir10.0_4.tar.xz"
ENGINE_DOWNLOAD_PATH="$ARTIFACTS_FOLDER/$ENGINE_DOWNLOAD_NAME"
ENGINE_URL="https://github.com/Sikarugir-App/Engines/releases/download/v1.0/$ENGINE_DOWNLOAD_NAME"

fail() {
    echo $@
    exit 1
}

printf "\e[1m"

echo "Quick setup for Kegworks"
echo "------------------------"

echo "=> Setting up wrapper and engine locations"

printf "\e[0m"
/bin/mkdir -p "$ARTIFACTS_FOLDER" || fail "Failed to create folder for temporary artifacts"
printf "\e[1m"

printf "\e[0m"
/bin/mkdir -p "$WRAPPER_FOLDER" || fail "Failed to create folder for wrappers"
/bin/mkdir -p "$ENGINE_FOLDER" || fail "Failed to create folder for engines"
printf "\e[1m"

echo "=> Downloading wrapper"

printf "\e[0m"
/usr/bin/curl -L "$WRAPPER_URL" > "$WRAPPER_DOWNLOAD_PATH" \
    || fail "Failed to download wrapper"

printf "\e[1m"

echo "=> Downloading engine"

printf "\e[0m"
/usr/bin/curl -L "$ENGINE_URL" > "$ENGINE_DOWNLOAD_PATH" \
    || fail "Failed to download engine"
printf "\e[1m"

echo "=> Installing wrapper..."

printf "\e[0m"
/usr/bin/tar xf "$WRAPPER_DOWNLOAD_PATH" --directory "$ARTIFACTS_FOLDER" || fail "Failed to extract wrapper from download"
/bin/mv "$ARTIFACTS_FOLDER/${WRAPPER_DOWNLOAD_NAME%.tar.xz}.app" "$WRAPPER_FOLDER" || fail "Failed to install wrapper"
printf "\e[1m"

echo "=> Installed wrapper"

echo "=> Installing engine..."

printf "\e[0m"
mv "$ENGINE_DOWNLOAD_PATH" "$ENGINE_FOLDER" || fail "Failed to install engine"
printf "\e[1m"

echo "=> Installed engine"

echo "=> Finished! You can now create a keg in kegtui"
printf "\e[0m"
