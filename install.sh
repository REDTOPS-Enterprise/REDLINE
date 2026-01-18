#!/bin/bash

# REDLINE Installer for Linux and macOS

set -e # Exit immediately if a command exits with a non-zero status.

# --- Determine User and Home Directory ---
# If the script is run with sudo, SUDO_USER will be set to the original user.
if [ -n "$SUDO_USER" ]; then
    USER_HOME=$(getent passwd "$SUDO_USER" | cut -d: -f6)
else
    USER_HOME=$HOME
fi

INSTALL_DIR="$USER_HOME/.redline"
BIN_DIR="/usr/local/bin"
REPO_URL="https://github.com/REDTOPS-Enterprise/REDLINE.git"

echo "Installing REDLINE for user: $SUDO_USER"
echo "Installation directory: $INSTALL_DIR"
echo ""

# --- Installation Steps ---

# 1. Remove old installation if it exists
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing existing installation..."
    rm -rf "$INSTALL_DIR"
fi
if [ -L "$BIN_DIR/redline" ]; then
    echo "Removing old symlink..."
    rm -f "$BIN_DIR/redline" # Use -f to avoid errors if it doesn't exist
fi

# 2. Clone the repository (as the original user to handle permissions correctly)
echo "Cloning REDLINE repository into $INSTALL_DIR..."
# We de-escalate to the user for the git clone
sudo -u "$SUDO_USER" git clone "$REPO_URL" "$INSTALL_DIR"

# 3. Make the main script executable
echo "Setting permissions..."
chmod +x "$INSTALL_DIR/redline.py"

# 4. Create a symlink (this is the only part that truly needs sudo)
echo "Creating symlink in $BIN_DIR..."
ln -s "$INSTALL_DIR/redline.py" "$BIN_DIR/redline"

# 5. Initialize the compiler core (as the original user)
echo "Initializing REDLINE compiler core (as user $SUDO_USER)..."
sudo -u "$SUDO_USER" "$INSTALL_DIR/redline.py" init

echo ""
echo "✅ REDLINE v1.0 installed successfully!"
echo "You can now use the 'redline' command anywhere on your system."
echo "(You may need to open a new terminal for the command to be available)."
