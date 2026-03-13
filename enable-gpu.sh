#!/bin/bash
# Enable GPU for Ochi Core
# This sets CUDA_VISIBLE_DEVICES to enable GPU detection

echo "🚀 Enabling GPU for Ochi Core..."
echo

# Check if CUDA is installed
if command -v nvcc &> /dev/null; then
    echo "✅ CUDA detected"
else
    echo "⚠️  CUDA not found in PATH"
    echo "💡 Make sure CUDA Toolkit is installed"
    echo
fi

# Set environment variable for this session
export CUDA_VISIBLE_DEVICES=0
echo "✅ Set CUDA_VISIBLE_DEVICES=0"
echo

echo "🎯 GPU is now enabled for this terminal session"
echo
echo To test GPU:
echo   cargo run -p ochi-core --example test_gpu_real
echo

# Run GPU test
echo "Running GPU test..."
cargo run -p ochi-core --example test_gpu_real

echo
echo "💡 Note: Environment variable is only set for this session"
echo "   To make it permanent, add to ~/.bashrc or /etc/environment
