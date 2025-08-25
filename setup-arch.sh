#!/bin/bash
# GridPointer setup script for Arch Linux + Hyprland

set -e

echo "🎯 GridPointer Setup for Arch Linux + Hyprland"
echo "=============================================="

# Check if running on Arch Linux
if ! command -v pacman &> /dev/null; then
    echo "❌ This script is designed for Arch Linux"
    exit 1
fi

# Check if running under Wayland
if [[ -z "$WAYLAND_DISPLAY" ]]; then
    echo "⚠️  Warning: WAYLAND_DISPLAY not set. Make sure you're running under Wayland."
fi

echo "📦 Installing dependencies..."
sudo pacman -S --needed \
    rust \
    cargo \
    wayland \
    wayland-protocols \
    git \
    make

# Add user to input group for device access
echo "👤 Adding user to input group..."
sudo usermod -a -G input "$USER"

# Build and install GridPointer
echo "🔨 Building GridPointer..."
cargo build --release

echo "📥 Installing binary..."
mkdir -p "$HOME/.local/bin"
cp target/release/gridpointer "$HOME/.local/bin/"
chmod +x "$HOME/.local/bin/gridpointer"

# Add ~/.local/bin to PATH if not already there
if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
    echo "🛤️  Adding ~/.local/bin to PATH..."
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc" 2>/dev/null || true
fi

# Create config directory
echo "⚙️  Creating configuration directory..."
mkdir -p "$HOME/.config/gridpointer"

# Install systemd service
echo "🔧 Installing systemd service..."
mkdir -p "$HOME/.config/systemd/user"
cp examples/gridpointer.service "$HOME/.config/systemd/user/"
systemctl --user daemon-reload

echo "✅ Installation complete!"
echo ""
echo "🎮 Next steps:"
echo "1. Log out and back in (or run 'newgrp input') to apply group membership"
echo "2. Start the service: systemctl --user start gridpointer"
echo "3. Enable auto-start: systemctl --user enable gridpointer"
echo "4. Check logs: journalctl --user -u gridpointer -f"
echo ""
echo "🎯 Default controls:"
echo "   Arrow keys: Move cursor"
echo "   Shift + arrows: Dash movement"
echo "   Space: Left click"
echo "   Escape: Quit daemon"
echo ""
echo "📝 Configuration: ~/.config/gridpointer/config.toml"
echo "📖 Documentation: https://github.com/yourusername/gridpointer"

