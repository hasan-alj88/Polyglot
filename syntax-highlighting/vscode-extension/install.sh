#!/bin/bash
# Aljam3 VSCode Extension Quick Install Script

set -e

EXTENSION_NAME="aljam3-language-support-0.1.0"
VSCODE_EXTENSIONS_DIR="$HOME/.vscode/extensions"
EXTENSION_DIR="$VSCODE_EXTENSIONS_DIR/$EXTENSION_NAME"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "========================================="
echo "Aljam3 VSCode Extension Installer"
echo "========================================="
echo ""

# Check if VSCode is installed
if ! command -v code &> /dev/null; then
    echo "⚠️  Warning: VSCode command 'code' not found in PATH"
    echo "   VSCode may not be installed, or you may need to add it to PATH"
    echo "   Continuing anyway..."
    echo ""
fi

# Create extensions directory if it doesn't exist
if [ ! -d "$VSCODE_EXTENSIONS_DIR" ]; then
    echo "📁 Creating VSCode extensions directory..."
    mkdir -p "$VSCODE_EXTENSIONS_DIR"
fi

# Remove existing installation
if [ -d "$EXTENSION_DIR" ]; then
    echo "🗑️  Removing existing installation..."
    rm -rf "$EXTENSION_DIR"
fi

# Copy extension files
echo "📦 Installing extension files..."
mkdir -p "$EXTENSION_DIR"
cp -r "$SCRIPT_DIR"/* "$EXTENSION_DIR/"

# Remove the install script from the installed extension
rm -f "$EXTENSION_DIR/install.sh"

echo ""
echo "✅ Installation complete!"
echo ""
echo "Extension installed to:"
echo "   $EXTENSION_DIR"
echo ""
echo "Next steps:"
echo "1. Reload VSCode window:"
echo "   - Press Ctrl+Shift+P (or Cmd+Shift+P on macOS)"
echo "   - Type: 'Developer: Reload Window'"
echo "   - Press Enter"
echo ""
echo "2. Test the extension:"
echo "   - Open a .aj3 file"
echo "   - Check language mode in bottom-right corner shows 'Aljam3'"
echo ""
echo "3. Test markdown highlighting:"
echo "   - Open test-samples/markdown-test.md"
echo "   - Verify code blocks are highlighted"
echo ""
echo "For uninstallation, run:"
echo "   rm -rf $EXTENSION_DIR"
echo ""
echo "========================================="
