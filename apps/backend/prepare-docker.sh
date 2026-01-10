#!/bin/bash
set -e

echo "🔧 Preparing knot_packages for Docker build..."

cd knot_packages

if [ -L "types" ]; then
    echo "📦 Copying types package..."
    rm types
    cp -r ../../../packages/types .
fi

if [ -L "utils" ]; then
    echo "📦 Copying utils package..."
    rm utils
    cp -r ../../../packages/utils .
fi

echo "✅ knot_packages prepared for Docker build"
