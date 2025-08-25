
# GridPointer 🎯

A high-performance daemon for Wayland/Hyprland that transforms your cursor into a smooth, game-like grid navigation experience.

## ✨ Features

- **🎮 Game-like Controls**: Navigate your cursor on a configurable grid with smooth easing
- **⚡ Ultra-smooth Motion**: 360 Hz update loop with easeOutCubic interpolation
- **🚀 Dash Support**: Quick movement across multiple cells
- **🎹 Multi-input**: Full keyboard and gamepad support via evdev
- **🖥️ Multi-monitor**: Seamless support for multiple displays
- **🔥 Hot-reload**: Configuration changes applied instantly without restart
- **💪 Lightweight**: Minimal resource usage (typically <50MB RAM)
- **🔧 Systemd Integration**: Runs as a user service with proper security

## 🚀 Quick Start

### Prerequisites (Arch Linux + Hyprland)

```bash
# Install dependencies
sudo pacman -S rust wayland wayland-protocols

# Make sure you're running under Wayland
echo $WAYLAND_DISPLAY  # Should output something like "wayland-1"
```

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/gridpointer.git
cd gridpointer

# Build and install
cargo build --release
mkdir -p ~/.local/bin
cp target/release/gridpointer ~/.local/bin/

# Create config directory
mkdir -p ~/.config/gridpointer

# Run directly (for testing)
gridpointer

# Or install as systemd service (recommended)
cp examples/gridpointer.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable gridpointer
systemctl --user start gridpointer
```

### Verify Installation

```bash
# Check if service is running
systemctl --user status gridpointer

# View logs
journalctl --user -u gridpointer -f
```

## ⚙️ Configuration

GridPointer uses `~/.config/gridpointer/config.toml` for configuration. If the file doesn't exist, it will be created with sensible defaults.

### Example Configuration

```toml
[grid]
cols = 20          # Number of grid columns
rows = 12          # Number of grid rows

[movement]
dash_cells = 5     # Number of cells to jump when dashing
tween_ms = 150     # Animation duration in milliseconds

[input]
keyboard_device = "/dev/input/event0"  # Optional: specific device path
gamepad_device = "/dev/input/event1"   # Optional: specific gamepad path

[display]
target_monitor = "eDP-1"  # Monitor name, or "auto" for primary
```

### Configuration Options

| Section | Key | Type | Default | Description |
|---------|-----|------|---------|-------------|
| `[grid]` | `cols` | u32 | 20 | Grid columns |
| `[grid]` | `rows` | u32 | 12 | Grid rows |
| `[movement]` | `dash_cells` | u32 | 5 | Dash distance |
| `[movement]` | `tween_ms` | u64 | 150 | Animation duration |
| `[input]` | `keyboard_device` | String? | auto-detect | Keyboard device path |
| `[input]` | `gamepad_device` | String? | auto-detect | Gamepad device path |
| `[display]` | `target_monitor` | String | "auto" | Target monitor |

## 🎮 Controls

### Keyboard (Default)

| Key | Action |
|-----|--------|
| **Arrow Keys** | Move to adjacent grid cell |
| **Shift + Arrow** | Dash movement (multi-cell jump) |
| **Space** | Left mouse click |
| **Escape** | Quit daemon |

### Gamepad

| Input | Action |
|-------|--------|
| **Left Stick** | Proportional movement (analog) |
| **A Button** | Left mouse click |
| **Start Button** | Quit daemon |

## 🏗️ Architecture

```
src/
├── main.rs      - Entry point, main loop at 360 Hz
├── config.rs    - Configuration with hot-reload via inotify
├── input.rs     - Keyboard/gamepad input via evdev
├── motion.rs    - Movement FSM with easeOutCubic interpolation
├── wl.rs        - Wayland virtual pointer integration
└── error.rs     - Centralized error handling

tests/           - Comprehensive unit tests
examples/        - Demo programs and systemd service
```

### Key Components

- **Main Loop**: Runs at 360 Hz (2.78ms per frame) for ultra-smooth motion
- **Motion Controller**: State machine handling movement with smooth easing
- **Input Manager**: Handles both keyboard and gamepad via evdev
- **Wayland Manager**: Virtual pointer control via zwlr_virtual_pointer_v1
- **Config Manager**: Hot-reload configuration without restart

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test config_tests
cargo test motion_tests

# Run with output
cargo test -- --nocapture

# Check formatting and linting
cargo fmt --check
cargo clippy -- -D warnings
```

## 📚 Examples

### Basic Demo

```bash
# Run the basic grid movement demo
cargo run --example grid_demo
```

### Dash Demo

```bash
# Run the dash movement demonstration
cargo run --example dash_demo
```

## 🔧 Development

### Building from Source

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run with logging
RUST_LOG=gridpointer=debug cargo run
```

### Adding New Features

The codebase is designed to be modular and extensible:

1. **New Input Devices**: Extend `src/input.rs`
2. **Movement Modes**: Add variants to `MotionEvent` in `src/motion.rs`
3. **Easing Functions**: Implement in `src/motion.rs`
4. **Display Features**: Extend `src/wl.rs`

### Code Style

- Follow Rust 2021 conventions
- Use `cargo fmt` for formatting
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality
- Document public APIs with rustdoc

## 🛠️ Troubleshooting

### Common Issues

**Service won't start:**
```bash
# Check logs
journalctl --user -u gridpointer -n 50

# Verify Wayland
echo $WAYLAND_DISPLAY

# Check permissions
ls -la /dev/input/
```

**No input devices found:**
```bash
# List available input devices
ls -la /dev/input/by-id/

# Check device permissions (you might need to add your user to input group)
sudo usermod -a -G input $USER
# Then log out and back in
```

**Cursor not moving:**
```bash
# Verify virtual pointer support in compositor
# For Hyprland, ensure you have wlroots-based virtual pointer support

# Check if running under correct Wayland session
loginctl show-session $(loginctl | grep $(whoami) | awk '{print $1}') -p Type
```

### Performance Tuning

For optimal performance:

1. **CPU Governor**: Use `performance` mode
   ```bash
   sudo cpupower frequency-set -g performance
   ```

2. **Process Priority**: Run with higher priority (optional)
   ```bash
   sudo nice -n -10 gridpointer
   ```

3. **Memory Lock**: Prevent swapping (optional)
   ```bash
   sudo setcap 'cap_ipc_lock=+ep' ~/.local/bin/gridpointer
   ```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests to our repository.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🙏 Acknowledgments

- [wlroots](https://gitlab.freedesktop.org/wlroots/wlroots) for virtual pointer protocol
- [Hyprland](https://hyprland.org/) for excellent Wayland compositor
- Rust community for amazing ecosystem

---

**Made with ❤️ for smooth cursor control**
