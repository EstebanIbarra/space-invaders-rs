#!/bin/bash

echo "Installing git hooks..."

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Copy git hooks
cp .githooks/* .git/hooks

# Make git hooks executable
chmod +x .git/hooks/*

echo "Git hooks installed."
exit 0
