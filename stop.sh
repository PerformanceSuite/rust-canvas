#!/bin/bash

echo "🛑 Stopping Audio Control Matrix..."

# Find and kill any running cargo/egui-test processes
pkill -f "egui-test" 2>/dev/null
pkill -f "cargo run" 2>/dev/null

echo "✅ Application stopped"