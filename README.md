# egui_kit

[![Crates.io](https://img.shields.io/crates/v/egui_kit.svg)](https://crates.io/crates/egui_kit)
[![Documentation](https://docs.rs/egui_kit/badge.svg)](https://docs.rs/egui_kit)
[![License](https://img.shields.io/crates/l/egui_kit.svg)](LICENSE-MIT)

Elegant theme system for [egui](https://github.com/emilk/egui) applications with **zero external dependencies**.

## Features

✨ **4 Beautiful Presets**: Modern Dark/Light, Nord, and Dracula themes
🎨 **Semantic Colors**: Use meaningful names like `primary`, `success`, `error`
🔄 **Runtime Switching**: Change themes without restarting
🔒 **Type Safety**: Compile-time checked, zero runtime errors
📦 **Zero Dependencies**: Only requires `egui`, no JSON files or external configs
📡 **BLE-Specific Colors**: Pre-defined colors for signal strength and device states

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
egui_kit = "0.1"
egui = "0.33"
eframe = "0.33"
```

Apply a theme on startup:

```rust
use egui_kit::{setup_theme, ThemePreset};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "My App",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // Apply theme on startup
            setup_theme(&cc.egui_ctx, ThemePreset::Dark);
            Ok(Box::new(MyApp::new()))
        }),
    )
}
```

## Available Themes

### Modern Dark (Default)
Clean, professional theme inspired by GitHub Dark and VS Code Dark+.
- **Background**: Deep blue-gray (#0F1729)
- **Primary**: Vibrant blue (#2563EB)
- **Best for**: Long coding sessions, night-time use

### Modern Light
High-contrast light theme with excellent readability.
- **Background**: Light gray-white (#F8FAFC)
- **Primary**: Blue (#2563EB)
- **Best for**: Daytime work, bright environments

### Nord
Minimal arctic-inspired theme, popular among developers.
- **Background**: Polar night (#2E3440)
- **Primary**: Frost blue (#5E81AC)
- **Best for**: Aesthetic minimalism, reduced visual noise

### Dracula
Vibrant dark theme with punchy, high-saturation colors.
- **Background**: Deep purple (#282A36)
- **Primary**: Purple (#BD93F9)
- **Best for**: High visual feedback, strong UI distinction

## Runtime Theme Switching

Add a theme selector to your UI:

```rust
use egui_kit::{ThemePreset, ThemeConfig, apply_theme};

struct MyApp {
    current_theme: ThemePreset,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("theme_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Theme:");
                for preset in ThemePreset::all() {
                    if ui.selectable_label(
                        self.current_theme == *preset,
                        preset.name()
                    ).clicked() {
                        self.current_theme = *preset;
                        let theme = ThemeConfig::from_preset(*preset);
                        apply_theme(ctx, &theme);
                    }
                }
            });
        });
    }
}
```

## Using Semantic Colors

Access theme colors by their semantic meaning:

```rust
use egui_kit::ThemeConfig;

let theme = ThemeConfig::from_preset(ThemePreset::Dark);
let colors = &theme.colors;

// Background colors
let bg = colors.background;
let surface = colors.surface;

// Text colors
let text = colors.text;
let text_muted = colors.text_secondary;

// Action colors
let primary = colors.primary;
let primary_hover = colors.primary_light;

// Semantic states
let success = colors.success;  // Green
let warning = colors.warning;  // Yellow
let error = colors.error;      // Red
let info = colors.info;        // Blue/Teal
```

## BLE-Specific Colors

Pre-configured colors for BLE signal strength visualization:

```rust
fn rssi_color(theme: &ThemeConfig, rssi: i16) -> egui::Color32 {
    match rssi {
        -50.. => theme.colors.ble_signal_strong,    // > -50 dBm (green)
        -70..-50 => theme.colors.ble_signal_good,   // -50 to -70 dBm (teal)
        -90..-70 => theme.colors.ble_signal_weak,   // -70 to -90 dBm (yellow)
        _ => theme.colors.ble_signal_very_weak,     // < -90 dBm (red)
    }
}

// Connection state colors
let connected = theme.colors.ble_connected;      // Green
let disconnected = theme.colors.ble_disconnected; // Gray
```

## Example

Run the interactive theme showcase:

```bash
cargo run --example showcase
```

This demonstrates:
- All 4 theme presets
- Complete color palette
- All egui widget states (buttons, inputs, checkboxes, etc.)
- Runtime theme switching

## Advanced Usage

### Custom Theme

Create a completely custom theme:

```rust
use egui_kit::{ThemeConfig, ThemeColors};
use egui::Color32;

let custom_theme = ThemeConfig {
    colors: ThemeColors {
        background: Color32::from_rgb(20, 20, 25),
        surface: Color32::from_rgb(30, 30, 35),
        text: Color32::from_rgb(240, 240, 245),
        primary: Color32::from_rgb(100, 150, 255),
        // ... configure all other colors
    },
};

egui_kit::apply_theme(&ctx, &custom_theme);
```

### Access Color Palette Directly

For fine-grained control:

```rust
use egui_kit::color_palette::{ColorPalette, Shade};

let palette = ColorPalette::modern_dark();

let very_dark = palette.gray(Shade::S900);
let medium = palette.gray(Shade::S500);
let very_light = palette.gray(Shade::S50);

let blue_500 = palette.blue(Shade::S500);
let green_600 = palette.green(Shade::S600);
```

## Design Philosophy

1. **Semantic over Abstract**: Use `theme.colors.primary` instead of `gray(S600)`
2. **Type Safety**: Compile-time checks prevent typos and invalid colors
3. **Zero Runtime Cost**: All colors are computed at compile time
4. **Consistency**: All themes follow the same color hierarchy
5. **Accessibility**: Carefully chosen contrast ratios for readability

## Architecture

```
egui_kit/
├── src/
│   ├── lib.rs              # Public API
│   ├── color_palette.rs    # Low-level color definitions
│   ├── theme_config.rs     # Semantic color mappings
│   └── presets.rs          # Pre-configured themes
├── examples/
│   └── showcase.rs         # Interactive demo
└── README.md               # This file
```

## Comparison with Other Approaches

| Feature | JSON Config | This Library |
|---------|-------------|--------------|
| Configuration | External file | Pure Rust |
| Dependencies | serde_json | Zero |
| Type Safety | Runtime parsing | Compile-time |
| Runtime Overhead | JSON parsing | Zero |
| IDE Support | Limited | Full autocomplete |
| Error Handling | Runtime errors | Compile errors |

## Minimum Supported Rust Version

Rust 1.70 or higher.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

Inspired by:
- [Tailwind CSS](https://tailwindcss.com/docs/customizing-colors) color system
- [GitHub Dark](https://primer.style/primitives/colors) theme
- [Nord Theme](https://www.nordtheme.com/)
- [Dracula Theme](https://draculatheme.com/)

Built with ❤️ for the [egui](https://github.com/emilk/egui) community.
