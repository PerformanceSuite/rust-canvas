#!/bin/bash

current_version=$(grep '^version = ' Cargo.toml | cut -d '"' -f 2)
echo "🎛️ Ev2 v$current_version - Audio Control Matrix Status"
echo "===================================="
echo ""

# Check if app bundle exists
if [ -d "Ev2.app" ]; then
    echo "✅ App Bundle: Ev2.app (ready to use)"
    
    # Check if installed in Applications
    if [ -d "/Applications/Ev2.app" ]; then
        echo "✅ Installed: /Applications/Ev2.app"
    else
        echo "📦 Not Installed: Run ./install_app.sh to install"
    fi
else
    echo "❌ App Bundle: Not built yet"
    echo "📝 Run ./build_app.sh to create app bundle"
fi

echo ""

# Check if any instances are running
if pgrep -f "egui-test\|Ev2" > /dev/null; then
    echo "🟢 Status: Ev2 is currently running"
    echo "🛑 To stop: ./stop.sh or press Ctrl+C in terminal"
else
    echo "⭕ Status: Ev2 is not running"
    echo "🚀 To start: ./start.sh or open Ev2.app"
fi

echo ""
echo "🔧 Quick Commands:"
echo "   ./build_app.sh    - Build native app"
echo "   ./install_app.sh  - Install to Applications"  
echo "   ./start.sh        - Start app (auto-detect mode)"
echo "   ./stop.sh         - Stop all instances"
echo "   open Ev2.app      - Launch app bundle"