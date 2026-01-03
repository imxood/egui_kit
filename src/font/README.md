# Font Loading System

Automatic system font loading with language detection for egui applications.

## Overview

This module provides:
- **Automatic language detection** using system locale
- **Font presets** for Chinese, English, Japanese, Korean
- **Runtime font switching** for language and manual selection
- **Minimal overhead** - only scans font list at startup, loads one font

## Features

- ✅ Auto-detect system language and load best font
- ✅ Priority-based font selection from presets
- ✅ Runtime language switching
- ✅ Runtime font switching (from system font list)
- ✅ Windows DirectWrite backend (~200KB binary size)
- ✅ Feature-gated (only enabled with `font-loading` feature)

## Usage

### Enable Feature

Add to your `Cargo.toml`:

```toml
[dependencies]
egui_kit = { version = "0.1", features = ["font-loading"] }
```

### Basic Usage (Auto-detect Language)

```rust
use egui_kit::font::FontManager;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "My App",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // Initialize font manager - auto-detects language
            let font_manager = FontManager::new(&cc.egui_ctx)?;
            println!("Loaded: {}", font_manager.current_font());

            Ok(Box::new(MyApp { font_manager }))
        }),
    )
}
```

### Switch Language at Runtime

```rust
use egui_kit::font::Language;

// In your UI code
if ui.button("Switch to Chinese").clicked() {
    font_manager.switch_language(ui.ctx(), Language::Chinese)?;
}
```

### Manual Font Selection

```rust
// Show all available fonts in a dropdown
egui::ComboBox::from_label("Font")
    .selected_text(font_manager.current_font())
    .show_ui(ui, |ui| {
        for font_name in font_manager.available_fonts() {
            if ui.selectable_label(false, font_name).clicked() {
                font_manager.switch_font(ui.ctx(), font_name)?;
            }
        }
    });
```

## Font Presets

### Chinese (按优先级)
1. Microsoft YaHei (微软雅黑)
2. SimHei (黑体)
3. Microsoft YaHei UI
4. SimSun (宋体)
5. NSimSun (新宋体)
6. KaiTi (楷体)

### English
1. Segoe UI (Windows 10/11 default)
2. Arial
3. Calibri
4. Tahoma
5. Verdana
6. Consolas

### Japanese (日本語)
1. Yu Gothic UI
2. Yu Gothic
3. Meiryo UI
4. Meiryo
5. MS Gothic
6. MS UI Gothic

### Korean (한국어)
1. Malgun Gothic
2. Gulim
3. Dotum
4. Batang

## Language Detection

The system uses `sys-locale` to detect the system language:

- `zh-*` → Chinese
- `ja-*` → Japanese
- `ko-*` → Korean
- Others → English (default)

## API Reference

### `FontManager`

Main entry point for font management.

```rust
pub struct FontManager {
    // ...
}

impl FontManager {
    /// Initialize with auto-detected language
    pub fn new(ctx: &egui::Context) -> Result<Self, FontError>;

    /// Get all system fonts
    pub fn available_fonts(&self) -> &[String];

    /// Get current language
    pub fn current_language(&self) -> Language;

    /// Get current font name
    pub fn current_font(&self) -> &str;

    /// Switch language (auto-selects best font)
    pub fn switch_language(
        &mut self,
        ctx: &egui::Context,
        language: Language,
    ) -> Result<(), FontError>;

    /// Switch to specific font
    pub fn switch_font(
        &mut self,
        ctx: &egui::Context,
        family_name: &str,
    ) -> Result<(), FontError>;
}
```

### `Language`

Supported languages.

```rust
pub enum Language {
    Chinese,
    English,
    Japanese,
    Korean,
}
```

### `FontError`

Error types for font operations.

```rust
pub enum FontError {
    ScanFailed(String),
    FontNotFound(String),
    NoFontInFamily(String),
    LoadFailed(String),
    NoSuitableFont(Language),
}
```

## Performance

### Startup Time
- Font list scan: ~50-100ms
- Single font load: ~10-30ms
- **Total**: ~60-130ms on first launch

### Memory Usage
- Font list: ~10-50KB (string names)
- Single font: ~2-10MB (TTF/OTF data)
- **Total**: ~2-10MB per loaded font

### Binary Size
- font-kit: ~50KB
- Dependencies: ~150KB
- **Total**: ~200KB (Windows DirectWrite backend)

## Limitations

1. **Windows Only** (currently)
   - Uses DirectWrite API
   - Linux/macOS support not tested

2. **Single Font Family**
   - Only loads one font at a time
   - Previous font is replaced when switching

3. **No Fallback Chain**
   - If preset fonts not found, returns error
   - User should handle fallback gracefully

## Examples

Run the complete demo:

```bash
cargo run -p egui_kit --example font_loading --features font-loading
```

## Troubleshooting

### Font Not Found Error

If you get `NoSuitableFont` error:

1. Check available fonts:
   ```rust
   let manager = FontManager::new(ctx)?;
   println!("Available: {:?}", manager.available_fonts());
   ```

2. Install missing fonts (Chinese example):
   - Download Microsoft YaHei
   - Or use fallback: `manager.switch_font(ctx, "SimSun")?`

### Language Detection Wrong

Override auto-detection:

```rust
// Manually set language after creation
font_manager.switch_language(ctx, Language::Chinese)?;
```

### Font Looks Blurry

Check egui's pixel ratio:

```rust
ctx.set_pixels_per_point(1.5); // Adjust for your display
```

## Architecture

```
FontManager
├── FontLoader (internal)
│   ├── SystemSource (font-kit)
│   └── scan_system_fonts() → Vec<String>
├── Language Detection (sys-locale)
└── egui Integration
    └── FontDefinitions + set_fonts()
```

## Related

- [font-kit documentation](https://docs.rs/font-kit)
- [sys-locale documentation](https://docs.rs/sys-locale)
- [egui font documentation](https://docs.rs/egui/latest/egui/struct.FontDefinitions.html)
