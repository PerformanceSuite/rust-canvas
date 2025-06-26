#!/bin/bash

echo "🚀 Quick System App Update"
echo "=========================="

# Auto-bump patch version
current_version=$(grep '^version = ' Cargo.toml | cut -d '"' -f 2)
IFS='.' read -ra VERSION_PARTS <<< "$current_version"
major=${VERSION_PARTS[0]}
minor=${VERSION_PARTS[1]}
patch=$((${VERSION_PARTS[2]} + 1))
new_version="$major.$minor.$patch"

echo "📈 Auto-bumping version: $current_version → $new_version"

# Update version
./bump_version.sh $new_version > /dev/null 2>&1

# Install to system
echo "📱 Installing to system tray..."
./install_app.sh > /dev/null 2>&1

echo "✅ System app updated to v$new_version"
echo "💡 You can now use the dock/system tray version with latest features"