#!/usr/bin/env bash
# Binnacle Installer for Linux / macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/TechTideOhio/binnacle/main/install.sh | sh

set -e

REPO="TechTideOhio/binnacle"
INSTALL_DIR="${HOME}/.local/bin"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux*)  PLATFORM="linux" ;;
  darwin*) PLATFORM="macos" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

echo "Installing Binnacle (TideOS Evaluator Agent)..."
mkdir -p "$INSTALL_DIR"

TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$TAG" ]; then
  echo "Fallback: cargo install..."
  cargo install --git "https://github.com/${REPO}"
  exit 0
fi

ARCHIVE="binnacle-${PLATFORM}-x86_64.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${TAG}/${ARCHIVE}"

TMP_DIR=$(mktemp -d)
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ARCHIVE"
tar -xzf "$TMP_DIR/$ARCHIVE" -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/binnacle"
rm -rf "$TMP_DIR"

echo "Binnacle installed to $INSTALL_DIR/binnacle."
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo "Add $INSTALL_DIR to your PATH in ~/.bashrc or ~/.zshrc:"
  echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi
