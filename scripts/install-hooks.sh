#!/bin/bash
# Install BANKO git hooks
# Usage: bash scripts/install-hooks.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Installing BANKO git hooks..."

# Set hooks path to .githooks/
git config core.hooksPath .githooks

# Make hooks executable
chmod +x "$PROJECT_DIR/.githooks/pre-commit"
chmod +x "$PROJECT_DIR/.githooks/pre-push"
chmod +x "$PROJECT_DIR/.githooks/commit-msg"

echo "Git hooks installed successfully."
echo "Hooks path: .githooks/"
echo "To bypass: git commit --no-verify"
