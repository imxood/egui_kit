//! Heading components (h1-h7) with easy customization
//!
//! This module provides heading components that can be easily customized
//! with colors and text wrapping through a builder pattern.
//!
//! # Example
//!
//! ```rust,no_run
//! use egui_kit::{h1, h2, h3};
//!
//! // Basic heading
//! ui.add(h1("Large Title"));
//!
//! // With color
//! ui.add(h2("Colored Title").color(egui::Color32::BLUE));
//!
//! // With wrapping
//! ui.add(h3("Long title that wraps").wrap());
//!
//! // Both color and wrapping
//! ui.add(h1("Customized").color(egui::Color32::RED).wrap());
//! ```

use egui::{Color32, RichText, TextStyle, Ui, Widget};

/// Heading component with customizable color and text wrapping
pub struct Heading {
    text: String,
    level: HeadingLevel,
    color: Option<Color32>,
    wrap: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeadingLevel {
    H1, // 32px
    H2, // 28px
    H3, // 24px
    H4, // 20px
    H5, // 16px
    H6, // 14px
    H7, // 12px
}

impl HeadingLevel {
    fn text_style(&self) -> TextStyle {
        match self {
            HeadingLevel::H1 => TextStyle::Name("h1".into()),
            HeadingLevel::H2 => TextStyle::Name("h2".into()),
            HeadingLevel::H3 => TextStyle::Name("h3".into()),
            HeadingLevel::H4 => TextStyle::Name("h4".into()),
            HeadingLevel::H5 => TextStyle::Name("h5".into()),
            HeadingLevel::H6 => TextStyle::Name("h6".into()),
            HeadingLevel::H7 => TextStyle::Name("h7".into()),
        }
    }
}

impl Heading {
    /// Set custom color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Enable text wrapping
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }
}

impl Widget for Heading {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let mut rich_text = RichText::new(self.text).text_style(self.level.text_style());

        if let Some(color) = self.color {
            rich_text = rich_text.color(color);
        }

        if self.wrap {
            ui.add(egui::Label::new(rich_text).wrap())
        } else {
            ui.label(rich_text)
        }
    }
}

// Factory functions for each heading level

/// Create h1 heading (32px)
pub fn h1(text: impl Into<String>) -> Heading {
    Heading {
        text: text.into(),
        level: HeadingLevel::H1,
        color: None,
        wrap: false,
    }
}

/// Create h2 heading (28px)
pub fn h2(text: impl Into<String>) -> Heading {
    Heading {
        text: text.into(),
        level: HeadingLevel::H2,
        color: None,
        wrap: false,
    }
}

/// Create h3 heading (24px)
pub fn h3(text: impl Into<String>) -> Heading {
    Heading {
        text: text.into(),
        level: HeadingLevel::H3,
        color: None,
        wrap: false,
    }
}

/// Create h4 heading (20px)
pub fn h4(text: impl Into<String>) -> Heading {
    Heading {
        text: text.into(),
        level: HeadingLevel::H4,
        color: None,
        wrap: false,
    }
}

/// Create h5 heading (16px)
pub fn h5(text: impl Into<String>) -> Heading {
    Heading {
        text: text.into(),
        level: HeadingLevel::H5,
        color: None,
        wrap: false,
    }
}

/// Create h6 heading (14px)
pub fn h6(text: impl Into<String>) -> Heading {
    Heading {
        text: text.into(),
        level: HeadingLevel::H6,
        color: None,
        wrap: false,
    }
}

/// Create h7 heading (12px)
pub fn h7(text: impl Into<String>) -> Heading {
    Heading {
        text: text.into(),
        level: HeadingLevel::H7,
        color: None,
        wrap: false,
    }
}

/// Extension trait for convenient heading usage
pub trait HeadingExt {
    /// Add h1 heading (32px)
    fn h1(&mut self, text: impl Into<String>) -> egui::Response;

    /// Add h2 heading (28px)
    fn h2(&mut self, text: impl Into<String>) -> egui::Response;

    /// Add h3 heading (24px)
    fn h3(&mut self, text: impl Into<String>) -> egui::Response;

    /// Add h4 heading (20px)
    fn h4(&mut self, text: impl Into<String>) -> egui::Response;

    /// Add h5 heading (16px)
    fn h5(&mut self, text: impl Into<String>) -> egui::Response;

    /// Add h6 heading (14px)
    fn h6(&mut self, text: impl Into<String>) -> egui::Response;

    /// Add h7 heading (12px)
    fn h7(&mut self, text: impl Into<String>) -> egui::Response;
}

impl HeadingExt for Ui {
    fn h1(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(h1(text))
    }

    fn h2(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(h2(text))
    }

    fn h3(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(h3(text))
    }

    fn h4(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(h4(text))
    }

    fn h5(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(h5(text))
    }

    fn h6(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(h6(text))
    }

    fn h7(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(h7(text))
    }
}
