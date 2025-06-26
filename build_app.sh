#!/bin/bash

echo "🔨 Building Ev2 - Audio Control Matrix App Bundle"
echo "================================================"

# Build the application in release mode
echo "📦 Building application..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Create app bundle structure
APP_NAME="Ev2"
BUNDLE_DIR="$APP_NAME.app"
CONTENTS_DIR="$BUNDLE_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo "📁 Creating app bundle structure..."
rm -rf "$BUNDLE_DIR"
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy the binary
echo "📄 Copying executable..."
cp target/release/egui-test "$MACOS_DIR/$APP_NAME"

# Create Info.plist
echo "📋 Creating Info.plist..."
cat > "$CONTENTS_DIR/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>Ev2</string>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>com.audiocontrol.ev2</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Ev2</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.music</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.12</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>LSUIElement</key>
    <false/>
</dict>
</plist>
EOF

# Copy icon if available or create iconset
if [ -f "assets/icons/ev2.icns" ]; then
    echo "📸 Copying existing icon..."
    cp "assets/icons/ev2.icns" "$RESOURCES_DIR/AppIcon.icns"
elif [ -d "assets/icons/ev2.iconset" ]; then
    echo "📸 Creating icon from iconset..."
    iconutil -c icns "assets/icons/ev2.iconset" -o "$RESOURCES_DIR/AppIcon.icns"
else
    echo "📸 Creating icon from PNG..."
    # Create iconset from our PNG file
    ICONSET_DIR="assets/icons/ev2.iconset"
    mkdir -p "$ICONSET_DIR"
    
    # Copy our basic icon to all required sizes
    cp assets/icon-256.png "$ICONSET_DIR/icon_256x256.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_256x256@2x.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_128x128.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_128x128@2x.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_32x32.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_32x32@2x.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_16x16.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_16x16@2x.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_512x512.png"
    cp assets/icon-256.png "$ICONSET_DIR/icon_512x512@2x.png"
    
    iconutil -c icns "$ICONSET_DIR" -o "$RESOURCES_DIR/AppIcon.icns" 2>/dev/null || {
        echo "⚠️  Could not create .icns file, copying PNG as fallback"
        cp assets/icon-256.png "$RESOURCES_DIR/AppIcon.png"
    }
fi

# Make the executable actually executable
chmod +x "$MACOS_DIR/$APP_NAME"

# Create a launch script for easier dock integration
echo "🚀 Creating launch script..."
cat > "launch_ev2.sh" << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
open Ev2.app
EOF
chmod +x launch_ev2.sh

echo ""
echo "✅ App bundle created successfully!"
echo "📱 Application: $BUNDLE_DIR"
echo ""
echo "🚀 To run the app:"
echo "   • Double-click: $BUNDLE_DIR"
echo "   • Command line: open $BUNDLE_DIR"
echo "   • Script: ./launch_ev2.sh"
echo ""
echo "📌 To keep in dock:"
echo "   1. Open the app"
echo "   2. Right-click the dock icon"
echo "   3. Select 'Options > Keep in Dock'"
echo ""
echo "📁 The app can also be moved to /Applications/ folder"