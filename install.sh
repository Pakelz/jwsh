#!/usr/bin/env bash

set -e

REPO="Pakelz/jwsh"
VERSION=${1:-latest}

if [ "$VERSION" == "latest" ]; then
  VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep -Po '"tag_name": "\K.*?(?=")')
fi

OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" = "Linux" ]; then
  FILE="jwsh-Linux.tar.gz"
else
  echo "OS $OS tidak didukung"
  exit 1
fi

URL="https://github.com/$REPO/releases/download/$VERSION/$FILE"

echo "📥 Downloading $URL"
curl -L "$URL" -o "$FILE"

echo "📦 Extracting..."
tar -xzf "$FILE"

echo "🚀 Installing to /usr/local/bin (need sudo)"
sudo mv jwsh /usr/local/bin/

echo "🧹 Cleaning up"
rm -f "$FILE"

echo "✅ Done! Try run: jwsh -h"
