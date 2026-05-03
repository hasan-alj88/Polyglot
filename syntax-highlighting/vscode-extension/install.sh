#!/bin/bash

# Navigate to the extension directory
cd "$(dirname "$0")"

echo "🧹 Cleaning up old builds..."
rm -f *.vsix

echo "📦 Installing dependencies..."
npm install || { echo "❌ npm install failed"; exit 1; }

echo "🔨 Compiling extension..."
npm run compile || { echo "❌ Compilation failed"; exit 1; }

echo "📦 Packaging new VSIX..."
vsce package || { echo "❌ Packaging failed"; exit 1; }

echo "🗑️ Uninstalling existing extension from Antigravity..."
antigravity --uninstall-extension aljam3-lang.aljam3-language-support --force > /dev/null 2>&1 || true

echo "🔥 Purging Antigravity extension cache..."
rm -rf ~/.antigravity/extensions/aljam3-lang.aljam3-language-support*

# Find the newly built VSIX file
VSIX_FILE=$(ls *.vsix | head -n 1)

if [ -z "$VSIX_FILE" ]; then
    echo "❌ No VSIX file found after packaging."
    exit 1
fi

echo "🚀 Installing $VSIX_FILE..."
antigravity --install-extension "$VSIX_FILE" --force

echo "✅ Clean installation complete! Please completely close all Antigravity windows and reopen them to clear the grammar cache."
