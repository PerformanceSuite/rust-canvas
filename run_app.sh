#!/bin/bash

# Quick start script for Audio Control Matrix

echo "🎛️ Ev2 - Audio Control Matrix - Drag & Drop Interface"
echo "======================================================"
echo ""
echo "Features:"
echo "• Drag widgets from palette to canvas"
echo "• Resize panels using bottom-right corner handles"
echo "• Alignment guides (pink=center, yellow=widget align)"
echo "• Right-click widgets to edit properties"
echo "• Interactive controls (knobs, toggles, sliders)"
echo ""
echo "Starting application..."
echo ""

# Check if app bundle exists, otherwise run with cargo
if [ -d "Ev2.app" ]; then
    echo "📱 Launching app bundle..."
    open Ev2.app
else
    echo "🔨 Running in development mode..."
    cargo run
fi