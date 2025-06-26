#!/bin/bash

# Version bumping script for Ev2

if [ $# -eq 0 ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.3.0"
    echo ""
    current_version=$(grep '^version = ' Cargo.toml | cut -d '"' -f 2)
    echo "Current version: $current_version"
    exit 1
fi

NEW_VERSION=$1

echo "🔄 Bumping version to $NEW_VERSION"

# Update Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

echo "✅ Updated Cargo.toml"

# Rebuild to ensure version is correct
echo "🔨 Rebuilding..."
cargo build --quiet

# Update app bundle if it exists
if [ -d "Ev2.app" ]; then
    echo "📱 Rebuilding app bundle..."
    ./build_app.sh
    echo "✅ App bundle updated"
fi

echo "🎉 Version bumped to $NEW_VERSION successfully!"
echo ""
echo "🚀 Next steps:"
echo "   • Test the new version: cargo run"
echo "   • Install app bundle: ./install_app.sh"
echo "   • Commit changes: git add . && git commit -m 'Bump version to $NEW_VERSION'"