#!/usr/bin/env bash
set -euo pipefail

REPO="${REPO:-vo1x/portdog}"
BIN_NAME="${BIN_NAME:-portdog}"
VERSION="${VERSION:-latest}"                     
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  os_tag="unknown-linux-gnu" ;;
  Darwin) os_tag="apple-darwin" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64) arch_tag="x86_64" ;;
  arm64|aarch64) arch_tag="aarch64" ;;
  *) echo "Unsupported arch: $ARCH"; exit 1 ;;
esac

if [ "$VERSION" = "latest" ]; then
  TAG="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep -oE '"tag_name":\s*"[^"]+' | cut -d'"' -f4)"
else
  TAG="$VERSION"
fi

ASSET="${BIN_NAME}-${TAG}-${arch_tag}-${os_tag}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading $URL ..."
curl -fsSL "$URL" -o "$TMPDIR/$ASSET"

echo "Installing to $INSTALL_DIR ..."
tar -C "$TMPDIR" -xzf "$TMPDIR/$ASSET"

INNER_DIR="$(tar -tzf "$TMPDIR/$ASSET" | head -n1 | cut -d/ -f1)"
install -m 0755 "$TMPDIR/$INNER_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"

echo "Installed: $INSTALL_DIR/$BIN_NAME"
"$INSTALL_DIR/$BIN_NAME" --version || true
