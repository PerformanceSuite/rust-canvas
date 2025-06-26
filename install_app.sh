#!/bin/bash

echo "📲 Installing Ev2 to Applications folder"
echo "========================================"

if [ ! -d "Ev2.app" ]; then
    echo "❌ Ev2.app not found! Run ./build_app.sh first."
    exit 1
fi

echo "📁 Copying Ev2.app to /Applications/"
cp -R Ev2.app /Applications/

if [ $? -eq 0 ]; then
    echo "✅ Ev2 installed successfully!"
    echo ""
    echo "🚀 You can now:"
    echo "   • Find 'Ev2' in Spotlight search"
    echo "   • Find 'Ev2' in Applications folder"
    echo "   • Launch from Launchpad"
    echo ""
    echo "📌 To keep in dock:"
    echo "   1. Launch the app from Applications"
    echo "   2. Right-click the dock icon"
    echo "   3. Select 'Options > Keep in Dock'"
    echo ""
    echo "🎛️ Enjoy your Audio Control Matrix!"
else
    echo "❌ Installation failed. You may need admin privileges."
    echo "💡 Try: sudo ./install_app.sh"
fi