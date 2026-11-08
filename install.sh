#!/usr/bin/env bash
# Binnacle Installer for Linux / macOS
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/TechTideOhio/binnacle/main/install.sh -o install.sh
#   bash install.sh

set -euo pipefail

REPO="TechTideOhio/binnacle"
INSTALL_DIR="${HOME}/.local/bin"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux*)  PLATFORM="linux" ;;
  darwin*) PLATFORM="macos" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64)  CPU="x86_64" ;;
  arm64|aarch64) CPU="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

echo "Installing Binnacle (TideOS Evaluator Agent) for ${PLATFORM}-${CPU}..."
mkdir -p "$INSTALL_DIR"

TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' || true)
if [ -z "$TAG" ]; then
  echo "Notice: Could not fetch latest release tag. Building via cargo install..."
  cargo install --git "https://github.com/${REPO}"
  exit 0
fi

ARCHIVE="binnacle-${PLATFORM}-${CPU}.tar.gz"
BASE_URL="https://github.com/${REPO}/releases/download/${TAG}"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

if curl -fsSL "${BASE_URL}/${ARCHIVE}" -o "$TMP_DIR/$ARCHIVE"; then
  # Fail closed: never fall back to an unverified binary.
  if ! curl -fsSL "${BASE_URL}/${ARCHIVE}.sha256" -o "$TMP_DIR/$ARCHIVE.sha256"; then
    echo "Error: checksum file ${ARCHIVE}.sha256 is missing for release ${TAG}; refusing to install."
    exit 1
  fi
  EXPECTED="$(awk '{print $1}' "$TMP_DIR/$ARCHIVE.sha256")"
  ACTUAL="$(sha256_of "$TMP_DIR/$ARCHIVE")"
  if [ "$EXPECTED" != "$ACTUAL" ]; then
    echo "Error: checksum mismatch for ${ARCHIVE} (expected ${EXPECTED}, got ${ACTUAL}); refusing to install."
    exit 1
  fi
  tar -xzf "$TMP_DIR/$ARCHIVE" -C "$INSTALL_DIR"
  chmod +x "$INSTALL_DIR/binnacle"
else
  echo "Notice: No ${ARCHIVE} asset in release ${TAG}. Falling back to cargo install..."
  cargo install --git "https://github.com/${REPO}"
  exit 0
fi

echo "Binnacle installed successfully to $INSTALL_DIR/binnacle."

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo "Note: $INSTALL_DIR is not yet in your PATH. Add it to ~/.bashrc or ~/.zshrc:"
  echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo "Verifying installation..."
"$INSTALL_DIR/binnacle" table > /dev/null
echo "Verification complete. Run 'binnacle doctor' to get started."
