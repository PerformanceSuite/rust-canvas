#!/bin/bash

echo "🎛️ Starting Ev2 - Audio Control Matrix..."
echo "Press Ctrl+C to stop the application"
echo ""

# Check if app bundle exists, otherwise run with cargo
if [ -d "Ev2.app" ]; then
    echo "📱 Launching app bundle..."
    open Ev2.app
else
    echo "🔨 Running in development mode..."
    echo "(Run ./build_app.sh to create app bundle)"
    cargo run
fi

echo ""
echo "✅ Application stopped"