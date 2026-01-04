use std::iter::once;

use egui::os::OperatingSystem;
use egui::{Atom, Atoms, IntoAtoms as _, ModifierNames, Modifiers};

use crate::components::basic::icon;

pub struct IconText;

impl IconText {
    pub fn from_keyboard_shortcut(
        os: OperatingSystem,
        shortcut: egui::KeyboardShortcut,
    ) -> Atoms<'static> {
        let egui::KeyboardShortcut {
            modifiers,
            logical_key,
        } = shortcut;

        let key_text = if os.is_mac() {
            logical_key.symbol_or_name()
        } else {
            logical_key.name()
        };
        Self::from_modifiers_and(os, modifiers, key_text)
    }

    pub fn from_modifiers_and(
        os: OperatingSystem,
        modifiers: Modifiers,
        icon: impl Into<Atom<'static>>,
    ) -> Atoms<'static> {
        if modifiers.is_none() {
            (icon.into()).into_atoms()
        } else {
            // macOS uses compact symbols for shortcuts without a `+`:
            if os.is_mac() {
                (Self::from_modifiers(os, modifiers), icon.into()).into_atoms()
            } else {
                (Self::from_modifiers(os, modifiers), "+", icon.into()).into_atoms()
            }
        }
    }

    /// Helper to add [`egui::Modifiers`] as text with icons.
    /// Will automatically show Cmd/Ctrl based on the OS.
    pub fn from_modifiers(os: OperatingSystem, modifiers: Modifiers) -> Atoms<'static> {
        let is_mac = os.is_mac();

        let names = if is_mac {
            ModifierNames::SYMBOLS
        } else {
            ModifierNames::NAMES
        };
        let text = names.format(&modifiers, is_mac);

        if is_mac {
            let mut atoms = Atoms::new(());
            for char in text.chars() {
                if char == '⌘' {
                    atoms.push_right(icon::COMMAND);
                } else if char == '⌃' {
                    atoms.push_right(icon::CONTROL);
                } else if char == '⇧' {
                    atoms.push_right(icon::SHIFT);
                } else if char == '⌥' {
                    atoms.push_right(icon::OPTION);
                } else {
                    // If there is anything else than the modifier symbols, just show the text.
                    return text.into_atoms();
                }
            }
            atoms
        } else {
            let mut vec: Vec<_> = text
                .split('+')
                // We want each + to be an extra item so the spacing looks nicer
                .flat_map(|item| once(item).chain(once("+")))
                .collect();
            vec.pop(); // Remove the last "+"

            vec.into()
        }
    }
}

/// Helper to show mouse buttons as text/icons.
pub struct MouseButtonText(pub egui::PointerButton);

impl From<MouseButtonText> for Atom<'_> {
    fn from(value: MouseButtonText) -> Self {
        match value.0 {
            egui::PointerButton::Primary => icon::LEFT_MOUSE_CLICK.into(),
            egui::PointerButton::Secondary => icon::RIGHT_MOUSE_CLICK.into(),
            egui::PointerButton::Middle => "middle mouse button".into(),
            egui::PointerButton::Extra1 => "extra 1 mouse button".into(),
            egui::PointerButton::Extra2 => "extra 2 mouse button".into(),
        }
    }
}
