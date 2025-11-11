#!/bin/bash
# Download and extract EVE Online Static Data Export (SDE)
# This script downloads the official EVE SDE from CCP's servers

set -e  # Exit on error

SDE_URL="https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip"
SDE_DIR="./sde_data"
ZIP_FILE="./sde.zip"

echo "=================================="
echo "EVE SDE Download Script"
echo "=================================="
echo ""

# Check if SDE directory already exists
if [ -d "$SDE_DIR" ]; then
    echo "⚠️  SDE directory already exists at $SDE_DIR"
    read -p "Do you want to delete it and re-download? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Removing existing SDE directory..."
        rm -rf "$SDE_DIR"
    else
        echo "Keeping existing SDE directory. Exiting."
        exit 0
    fi
fi

# Download the SDE
echo "📥 Downloading EVE SDE from CCP servers..."
echo "   URL: $SDE_URL"
echo "   Size: ~600MB (this may take a few minutes)"
echo ""

if command -v wget &> /dev/null; then
    wget -O "$ZIP_FILE" "$SDE_URL"
elif command -v curl &> /dev/null; then
    curl -L -o "$ZIP_FILE" "$SDE_URL"
else
    echo "❌ Error: Neither wget nor curl is available."
    echo "   Please install wget or curl to download the SDE."
    exit 1
fi

# Verify the download
if [ ! -f "$ZIP_FILE" ]; then
    echo "❌ Error: Download failed. ZIP file not found."
    exit 1
fi

echo ""
echo "✅ Download complete!"
echo ""

# Extract the SDE
echo "📦 Extracting SDE data..."
if command -v unzip &> /dev/null; then
    unzip -q "$ZIP_FILE" -d "$SDE_DIR"
else
    echo "❌ Error: unzip is not available."
    echo "   Please install unzip to extract the SDE."
    rm -f "$ZIP_FILE"
    exit 1
fi

# Clean up the ZIP file
echo "🧹 Cleaning up..."
rm -f "$ZIP_FILE"

# Verify extraction
if [ -d "$SDE_DIR/fsd" ] && [ -d "$SDE_DIR/bsd" ]; then
    echo ""
    echo "✅ SDE extraction complete!"
    echo ""
    echo "📊 SDE Statistics:"
    echo "   Location: $SDE_DIR"
    echo "   Size: $(du -sh "$SDE_DIR" | cut -f1)"
    echo ""
    echo "📁 Key files:"
    if [ -f "$SDE_DIR/fsd/typeMaterials.yaml" ]; then
        echo "   ✓ typeMaterials.yaml (reprocessing data)"
    fi
    if [ -f "$SDE_DIR/fsd/types.yaml" ]; then
        echo "   ✓ types.yaml (item definitions)"
    fi
    if [ -f "$SDE_DIR/fsd/blueprints.yaml" ]; then
        echo "   ✓ blueprints.yaml (manufacturing data)"
    fi
    if [ -f "$SDE_DIR/bsd/invNames.yaml" ]; then
        echo "   ✓ invNames.yaml (item names)"
    fi
    echo ""
    echo "✅ Ready to run: python3 scripts/import_sde.py"
else
    echo "❌ Error: SDE extraction appears incomplete."
    echo "   Expected directories not found in $SDE_DIR"
    exit 1
fi
