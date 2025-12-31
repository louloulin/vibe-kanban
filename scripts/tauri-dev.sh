#!/bin/bash
# Tauri development startup script

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Start Vite dev server in the background
echo "Starting Vite dev server..."
cd frontend
pnpm run dev &
VITE_PID=$!
echo "Vite dev server started (PID: $VITE_PID)"

# Go back to project root for Tauri
cd "$PROJECT_ROOT"

# Wait for Vite to start
echo "Waiting for Vite to start..."
sleep 5

# Start Tauri
echo "Starting Tauri..."
pnpm exec tauri dev

# Cleanup on exit
kill $VITE_PID 2>/dev/null
