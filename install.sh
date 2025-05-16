#!/usr/bin/env zsh

set -e

ARCH="$(uname -m)"
case "$ARCH" in
  x86_64) ARCH_ID="x86_64-linux" ;;
  aarch64) ARCH_ID="aarch64-linux" ;;
  *) echo "Unsupported architecture: $ARCH" && exit 1 ;;
esac

TOOLS_DIR="$HOME/.tools"
mkdir -p "$TOOLS_DIR"
cd "$TOOLS_DIR"

# Fetch latest version from GitHub
LATEST_VERSION=$(curl -s https://api.github.com/repos/donhk/rushstr/releases/latest | grep tag_name | cut -d '"' -f 4)

if [[ -z "$LATEST_VERSION" ]]; then
  echo "❌ Could not determine the latest version" && exit 1
fi

FILENAME="rushstr-${LATEST_VERSION}-${ARCH_ID}.tar.xz"
URL="https://github.com/donhk/rushstr/releases/download/${LATEST_VERSION}/${FILENAME}"

echo "⬇️ Downloading $FILENAME..."
curl -LO "$URL"

echo "📦 Extracting..."
tar -xf "$FILENAME"
rm "$FILENAME"

# Add to PATH (permanent via ~/.zshrc)
if ! grep -q "$TOOLS_DIR" ~/.zshrc; then
  echo "\n# Add rushstr to PATH" >> ~/.zshrc
  echo "export PATH=\"$TOOLS_DIR:\$PATH\"" >> ~/.zshrc
  echo "🔧 Added rushstr to ~/.zshrc"
fi

# Generate shell integration
"$TOOLS_DIR/rushstr" --zsh-shell-conf

echo "✅ rushstr installed!"
echo "ℹ️ Run 'source ~/.zshrc' to activate rushstr keybinding in this terminal."
