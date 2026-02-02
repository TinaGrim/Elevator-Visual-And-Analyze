#!/bin/bash
# Script to run the Elevator GUI application

echo "🏢 Starting Elevator Visual and Analyze GUI..."
echo ""
echo "Controls:"
echo "  - Click the 'Start' checkbox to begin simulation"
echo "  - The elevator will automatically move between floors"
echo "  - Click on the elevator to toggle door open/close"
echo ""
echo "Running application..."
echo ""

cargo run --release
