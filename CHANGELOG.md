# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-21

### Added
- Initial release
- 4 preset themes: Modern Dark, Modern Light, Nord, Dracula
- Semantic color system with 52 named colors
- Runtime theme switching support
- Zero external dependencies (only egui required)
- Type-safe API with compile-time checks
- BLE-specific colors for signal strength and device states
- Complete color palettes with 10 shades per color
- Interactive showcase example
- Comprehensive documentation and README

### Features
- `setup_theme()` - Quick theme application on startup
- `apply_theme()` - Runtime theme switching
- `ThemePreset` - Enum for all preset themes
- `ThemeConfig` - Customizable theme configuration
- `ColorPalette` - Low-level color definitions
- `ThemeColors` - Semantic color mappings

[0.1.0]: https://github.com/yourusername/egui_kit/releases/tag/v0.1.0
