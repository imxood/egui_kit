# Elegant egui Theme System

A modern, type-safe theme system for egui applications with zero external dependencies.

## Features

- **4 Beautiful Presets**: Modern Dark/Light, Nord, and Dracula themes
- **Semantic Colors**: Use meaningful names like `primary`, `success`, `error` instead of abstract color codes
- **BLE-Specific Colors**: Pre-defined colors for RSSI signal strength indicators
- **Runtime Switching**: Change themes without restarting the application
- **Type Safety**: No string-based color lookups, all compile-time checked
- **Zero Dependencies**: No JSON files or external config, everything in pure Rust

## Quick Start

### Basic Usage

In your `main.rs`:

```rust
mod theme;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "My App",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // Apply theme on startup
            theme::setup_theme(&cc.egui_ctx, theme::ThemePreset::Dark);
            Ok(Box::new(MyApp::new()))
        }),
    )
}
```

### Runtime Theme Switching

Add theme selection to your app:

```rust
use crate::theme::{ThemePreset, apply_theme, ThemeConfig};

pub struct MyApp {
    current_theme: ThemePreset,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

## Available Themes

### Modern Dark (Default)
Inspired by GitHub Dark and VS Code Dark+. Clean, professional, and easy on the eyes.

```rust
theme::setup_theme(&ctx, theme::ThemePreset::Dark);
```

### Modern Light
A light variant with excellent readability for daytime use.

```rust
theme::setup_theme(&ctx, theme::ThemePreset::Light);
```

### Nord
Minimal, elegant arctic-inspired theme. Popular among developers.

```rust
theme::setup_theme(&ctx, theme::ThemePreset::Nord);
```

### Dracula
Vibrant dark theme with punchy colors. Great for making UI elements stand out.

```rust
theme::setup_theme(&ctx, theme::ThemePreset::Dracula);
```

## Using Semantic Colors

The theme system provides semantic color names for common UI patterns:

```rust
use crate::theme::ThemeConfig;

let theme = ThemeConfig::from_preset(ThemePreset::Dark);

// Background colors
let bg = theme.colors.background;
let surface = theme.colors.surface;

// Text colors
let text = theme.colors.text;
let text_muted = theme.colors.text_secondary;

// Action colors
let primary = theme.colors.primary;
let primary_hover = theme.colors.primary_light;

// Semantic states
let success = theme.colors.success;  // Green
let warning = theme.colors.warning;  // Yellow
let error = theme.colors.error;      // Red
let info = theme.colors.info;        // Blue/Teal

// Borders
let border = theme.colors.border;
let divider = theme.colors.divider;
```

## BLE-Specific Colors

Pre-configured colors for BLE signal strength visualization:

```rust
fn rssi_color(theme: &ThemeConfig, rssi: i16) -> Color32 {
    match rssi {
        -90.. => theme.colors.ble_signal_strong,    // > -90 dBm
        -70..-90 => theme.colors.ble_signal_good,   // -70 to -90 dBm
        -50..-70 => theme.colors.ble_signal_weak,   // -50 to -70 dBm
        _ => theme.colors.ble_signal_very_weak,     // < -50 dBm
    }
}

// Connection state colors
let connected = theme.colors.ble_connected;      // Green
let disconnected = theme.colors.ble_disconnected; // Gray
```

## Custom Theme Configuration

You can also create a completely custom theme:

```rust
use crate::theme::{ThemeConfig, ThemeColors};
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

apply_theme(&ctx, &custom_theme);
```

## Advanced: Color Palette Access

For fine-grained control, you can access the full color palette:

```rust
use crate::theme::color_palette::{ColorPalette, Shade};

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
theme/
├── mod.rs              # Public API and theme application
├── color_palette.rs    # Low-level color definitions
├── theme_config.rs     # Semantic color mappings
├── presets.rs          # Pre-configured themes
└── README.md           # This file
```

## Comparison with im_auto2

This theme system is inspired by the im_auto2 project but with significant improvements:

| Feature | im_auto2 | This Theme System |
|---------|----------|-------------------|
| Configuration | JSON file | Pure Rust |
| Dependencies | serde_json | None |
| Type Safety | String paths | Compile-time types |
| Runtime Overhead | JSON parsing | Zero |
| Extensibility | Modify JSON | Implement trait |
| IDE Support | Limited | Full autocomplete |

## Future Enhancements

- [ ] Theme persistence (save user preference)
- [ ] Smooth theme transitions with animations
- [ ] High contrast mode for accessibility
- [ ] Custom font family per theme
- [ ] Color blindness-safe variants
- [ ] Export theme to CSS/JSON for docs

## License

Same as the parent project.
