# Contributing to GridPointer

Thank you for your interest in contributing to GridPointer! This document provides guidelines and information for contributors.

## 🚀 Getting Started

1. **Fork the repository**
2. **Clone your fork**: `git clone https://github.com/yourusername/gridpointer.git`
3. **Create a feature branch**: `git checkout -b feature/amazing-feature`
4. **Set up development environment**: See [Development Setup](#development-setup)

## 🛠️ Development Setup

### Prerequisites

- Rust 2021 edition or later
- Wayland development libraries
- Input device access (user in `input` group)

### Setup Commands

```bash
# Install development dependencies
sudo pacman -S rust cargo wayland wayland-protocols

# Clone and setup
git clone https://github.com/yourusername/gridpointer.git
cd gridpointer

# Run development checks
make dev-check

# Run with debug logging
make dev-run
```

## 📝 Code Style

### Rust Guidelines

- Follow official Rust style guidelines
- Use `cargo fmt` for formatting
- Ensure `cargo clippy` passes without warnings
- Add documentation for public APIs
- Write unit tests for new functionality

### Commit Messages

Use conventional commit format:

```
feat: add gamepad analog stick support
fix: resolve cursor jumping on multi-monitor setups
docs: update installation instructions for Fedora
test: add integration tests for motion controller
perf: optimize easing calculations for 360Hz updates
```

### Code Organization

```
src/
├── main.rs      - Entry point and coordination
├── config.rs    - Configuration management
├── input.rs     - Input device handling
├── motion.rs    - Movement logic and easing
├── wl.rs        - Wayland protocol integration
└── error.rs     - Error handling
```

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test module
cargo test config_tests

# With coverage
make test-coverage

# Benchmarks
make bench
```

### Test Categories

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test component interactions
3. **Performance Tests**: Benchmark critical paths
4. **Hardware Tests**: Test with actual input devices

### Writing Tests

- Test both happy path and error conditions
- Use descriptive test names
- Add tests for bug fixes
- Mock external dependencies when possible

## 🐛 Bug Reports

### Before Reporting

1. Check existing issues
2. Test with latest version
3. Try minimal reproduction case

### Bug Report Template

```markdown
**Environment:**
- OS: Arch Linux
- Compositor: Hyprland 0.32.0
- GridPointer version: 0.1.0

**Description:**
Brief description of the issue

**Steps to Reproduce:**
1. Step one
2. Step two
3. See error

**Expected Behavior:**
What should happen

**Actual Behavior:**
What actually happens

**Logs:**
```
journalctl --user -u gridpointer -n 50
```

**Additional Context:**
Any other relevant information
```

## 💡 Feature Requests

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
Clear description of the problem

**Describe the solution you'd like**
Clear description of desired functionality

**Describe alternatives you've considered**
Other solutions you've thought about

**Additional context**
Mockups, examples, or related issues
```

## 🔄 Pull Request Process

### Before Submitting

1. **Test thoroughly**: Ensure all tests pass
2. **Check formatting**: Run `make fmt-check`
3. **Lint code**: Run `make clippy`
4. **Update documentation**: If adding features
5. **Add tests**: For new functionality

### PR Template

```markdown
**Description**
Brief description of changes

**Type of Change**
- [ ] Bug fix
- [ ] New feature  
- [ ] Breaking change
- [ ] Documentation update

**Testing**
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

**Checklist**
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added for new functionality
```

### Review Process

1. **Automated checks**: CI must pass
2. **Code review**: At least one maintainer approval
3. **Testing**: Verify functionality works as expected
4. **Merge**: Squash commits for clean history

## 🏗️ Architecture Guidelines

### Core Principles

1. **Modularity**: Keep components loosely coupled
2. **Performance**: Maintain 360Hz update rate
3. **Safety**: Handle errors gracefully
4. **Extensibility**: Design for future enhancements

### Performance Considerations

- **Hot paths**: Optimize critical update loops
- **Memory allocation**: Minimize in update loops
- **System calls**: Batch when possible
- **Async operations**: Don't block main loop

## 📚 Documentation

### Code Documentation

```rust
/// Brief description of function
///
/// More detailed explanation if needed
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```
/// let result = function(param);
/// assert_eq!(result, expected);
/// ```
pub fn function(param: Type) -> ReturnType {
    // implementation
}
```

### README Updates

- Keep installation instructions current
- Update feature list for new capabilities
- Add configuration examples
- Update troubleshooting section

## 🎯 Roadmap and Priorities

### High Priority

- Performance optimizations
- Bug fixes
- Documentation improvements
- Hardware compatibility

### Medium Priority

- New input devices
- Additional easing functions
- Configuration enhancements
- Multi-monitor improvements

### Low Priority

- UI/GUI configuration tool
- Scripting support
- Plugin system
- Advanced gaming features

## 🤝 Community

### Communication

- **Issues**: Bug reports and feature requests
- **Discussions**: General questions and ideas
- **Matrix**: Real-time chat (if available)

### Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors will be acknowledged in:
- README.md contributors section
- Release notes
- Project documentation

Thank you for contributing to GridPointer! 🎯
`
