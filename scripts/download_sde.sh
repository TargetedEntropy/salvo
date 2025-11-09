#!/bin/bash

# Script to download EVE Online SDE (Static Data Export)

echo "=== EVE Online SDE Downloader ==="
echo ""

SDE_DIR="./sde"
SDE_URL="https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip"

echo "This script will download the EVE SDE to: $SDE_DIR"
echo "Download URL: $SDE_URL"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    echo "Cancelled."
    exit 1
fi

# Create directory
mkdir -p "$SDE_DIR"

# Download SDE
echo "Downloading SDE (this may take several minutes)..."
curl -L "$SDE_URL" -o "$SDE_DIR/sde.zip"

if [ $? -ne 0 ]; then
    echo "Error: Download failed"
    exit 1
fi

# Unzip
echo "Extracting SDE..."
unzip -q "$SDE_DIR/sde.zip" -d "$SDE_DIR"

if [ $? -ne 0 ]; then
    echo "Error: Extraction failed"
    exit 1
fi

# Cleanup zip
rm "$SDE_DIR/sde.zip"

echo ""
echo "✓ SDE downloaded and extracted to: $SDE_DIR"
echo ""
echo "To import into database:"
echo "  cargo run --bin import_sde -- --sde-path $SDE_DIR"
echo ""
