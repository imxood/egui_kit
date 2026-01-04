// Color palette system with multiple shades
//
// Inspired by Tailwind CSS color system (50-950 scale)
// but simplified for egui use cases

use egui::Color32;

/// Color shade intensity (0 = lightest, 900 = darkest)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shade {
    S50,
    S100,
    S200,
    S300,
    S400,
    S500,
    S600,
    S700,
    S800,
    S900,
}

/// A complete color palette with multiple shades
///
/// Each color has 10 shades from lightest (50) to darkest (900)
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub gray: [Color32; 10],
    pub blue: [Color32; 10],
    pub green: [Color32; 10],
    pub red: [Color32; 10],
    pub yellow: [Color32; 10],
    pub purple: [Color32; 10],
    pub teal: [Color32; 10],
}

impl ColorPalette {
    /// Get color by shade index
    #[inline]
    pub fn get(&self, colors: &[Color32; 10], shade: Shade) -> Color32 {
        colors[shade as usize]
    }

    /// Get gray shade
    #[inline]
    pub fn gray(&self, shade: Shade) -> Color32 {
        self.get(&self.gray, shade)
    }

    /// Get blue shade
    #[inline]
    pub fn blue(&self, shade: Shade) -> Color32 {
        self.get(&self.blue, shade)
    }

    /// Get green shade
    #[inline]
    pub fn green(&self, shade: Shade) -> Color32 {
        self.get(&self.green, shade)
    }

    /// Get red shade
    #[inline]
    pub fn red(&self, shade: Shade) -> Color32 {
        self.get(&self.red, shade)
    }

    /// Get yellow shade
    #[inline]
    pub fn yellow(&self, shade: Shade) -> Color32 {
        self.get(&self.yellow, shade)
    }

    /// Get purple shade
    #[inline]
    pub fn purple(&self, shade: Shade) -> Color32 {
        self.get(&self.purple, shade)
    }

    /// Get teal shade
    #[inline]
    pub fn teal(&self, shade: Shade) -> Color32 {
        self.get(&self.teal, shade)
    }

    /// Modern dark palette (inspired by GitHub Dark + Tailwind)
    pub fn modern_dark() -> Self {
        Self {
            // Gray (neutral slate tones)
            gray: [
                Color32::from_rgb(248, 250, 252), // 50
                Color32::from_rgb(241, 245, 249), // 100
                Color32::from_rgb(226, 232, 240), // 200
                Color32::from_rgb(203, 213, 225), // 300
                Color32::from_rgb(148, 163, 184), // 400
                Color32::from_rgb(100, 116, 139), // 500
                Color32::from_rgb(71, 85, 105),   // 600
                Color32::from_rgb(51, 65, 85),    // 700
                Color32::from_rgb(30, 41, 59),    // 800
                Color32::from_rgb(15, 23, 42),    // 900
            ],
            // Blue (primary action color)
            blue: [
                Color32::from_rgb(239, 246, 255), // 50
                Color32::from_rgb(219, 234, 254), // 100
                Color32::from_rgb(191, 219, 254), // 200
                Color32::from_rgb(147, 197, 253), // 300
                Color32::from_rgb(96, 165, 250),  // 400
                Color32::from_rgb(59, 130, 246),  // 500
                Color32::from_rgb(37, 99, 235),   // 600
                Color32::from_rgb(29, 78, 216),   // 700
                Color32::from_rgb(30, 64, 175),   // 800
                Color32::from_rgb(30, 58, 138),   // 900
            ],
            // Green (success states)
            green: [
                Color32::from_rgb(240, 253, 244), // 50
                Color32::from_rgb(220, 252, 231), // 100
                Color32::from_rgb(187, 247, 208), // 200
                Color32::from_rgb(134, 239, 172), // 300
                Color32::from_rgb(74, 222, 128),  // 400
                Color32::from_rgb(34, 197, 94),   // 500
                Color32::from_rgb(22, 163, 74),   // 600
                Color32::from_rgb(21, 128, 61),   // 700
                Color32::from_rgb(22, 101, 52),   // 800
                Color32::from_rgb(20, 83, 45),    // 900
            ],
            // Red (error states)
            red: [
                Color32::from_rgb(254, 242, 242), // 50
                Color32::from_rgb(254, 226, 226), // 100
                Color32::from_rgb(254, 202, 202), // 200
                Color32::from_rgb(252, 165, 165), // 300
                Color32::from_rgb(248, 113, 113), // 400
                Color32::from_rgb(239, 68, 68),   // 500
                Color32::from_rgb(220, 38, 38),   // 600
                Color32::from_rgb(185, 28, 28),   // 700
                Color32::from_rgb(153, 27, 27),   // 800
                Color32::from_rgb(127, 29, 29),   // 900
            ],
            // Yellow (warning states)
            yellow: [
                Color32::from_rgb(254, 252, 232), // 50
                Color32::from_rgb(254, 249, 195), // 100
                Color32::from_rgb(254, 240, 138), // 200
                Color32::from_rgb(253, 224, 71),  // 300
                Color32::from_rgb(250, 204, 21),  // 400
                Color32::from_rgb(234, 179, 8),   // 500
                Color32::from_rgb(202, 138, 4),   // 600
                Color32::from_rgb(161, 98, 7),    // 700
                Color32::from_rgb(133, 77, 14),   // 800
                Color32::from_rgb(113, 63, 18),   // 900
            ],
            // Purple (accent color)
            purple: [
                Color32::from_rgb(250, 245, 255), // 50
                Color32::from_rgb(243, 232, 255), // 100
                Color32::from_rgb(233, 213, 255), // 200
                Color32::from_rgb(216, 180, 254), // 300
                Color32::from_rgb(192, 132, 252), // 400
                Color32::from_rgb(168, 85, 247),  // 500
                Color32::from_rgb(147, 51, 234),  // 600
                Color32::from_rgb(126, 34, 206),  // 700
                Color32::from_rgb(107, 33, 168),  // 800
                Color32::from_rgb(88, 28, 135),   // 900
            ],
            // Teal (info states, BLE signal indicator)
            teal: [
                Color32::from_rgb(240, 253, 250), // 50
                Color32::from_rgb(204, 251, 241), // 100
                Color32::from_rgb(153, 246, 228), // 200
                Color32::from_rgb(94, 234, 212),  // 300
                Color32::from_rgb(45, 212, 191),  // 400
                Color32::from_rgb(20, 184, 166),  // 500
                Color32::from_rgb(13, 148, 136),  // 600
                Color32::from_rgb(15, 118, 110),  // 700
                Color32::from_rgb(17, 94, 89),    // 800
                Color32::from_rgb(19, 78, 74),    // 900
            ],
        }
    }

    /// Light palette for light theme
    pub fn modern_light() -> Self {
        Self {
            // Inverted shades for light theme
            gray: [
                Color32::from_rgb(15, 23, 42),    // 50 (darkest in light mode)
                Color32::from_rgb(30, 41, 59),    // 100
                Color32::from_rgb(51, 65, 85),    // 200
                Color32::from_rgb(71, 85, 105),   // 300
                Color32::from_rgb(100, 116, 139), // 400
                Color32::from_rgb(148, 163, 184), // 500
                Color32::from_rgb(203, 213, 225), // 600
                Color32::from_rgb(226, 232, 240), // 700
                Color32::from_rgb(241, 245, 249), // 800
                Color32::from_rgb(248, 250, 252), // 900 (lightest in light mode)
            ],
            // Keep other colors the same (just reference from different shades)
            blue: Self::modern_dark().blue,
            green: Self::modern_dark().green,
            red: Self::modern_dark().red,
            yellow: Self::modern_dark().yellow,
            purple: Self::modern_dark().purple,
            teal: Self::modern_dark().teal,
        }
    }

    /// Nord theme palette (popular in developer tools)
    pub fn nord() -> Self {
        Self {
            // Nord Polar Night + Snow Storm
            gray: [
                Color32::from_rgb(236, 239, 244), // Snow Storm 0
                Color32::from_rgb(229, 233, 240), // Snow Storm 1
                Color32::from_rgb(216, 222, 233), // Snow Storm 2
                Color32::from_rgb(143, 157, 180), // Frost middle
                Color32::from_rgb(129, 161, 193), // Frost 2
                Color32::from_rgb(94, 129, 172),  // Frost 1
                Color32::from_rgb(76, 86, 106),   // Polar Night 2
                Color32::from_rgb(67, 76, 94),    // Polar Night 1
                Color32::from_rgb(59, 66, 82),    // Polar Night 0
                Color32::from_rgb(46, 52, 64),    // Polar Night darkest
            ],
            // Nord Frost
            blue: [
                Color32::from_rgb(216, 222, 233),
                Color32::from_rgb(194, 206, 224),
                Color32::from_rgb(163, 182, 211),
                Color32::from_rgb(143, 157, 180),
                Color32::from_rgb(136, 192, 208), // Nord 8
                Color32::from_rgb(129, 161, 193), // Nord 9
                Color32::from_rgb(94, 129, 172),  // Nord 10
                Color32::from_rgb(81, 119, 162),
                Color32::from_rgb(70, 100, 145),
                Color32::from_rgb(59, 66, 82),
            ],
            // Nord Aurora (Green)
            green: [
                Color32::from_rgb(230, 244, 235),
                Color32::from_rgb(208, 233, 218),
                Color32::from_rgb(186, 222, 201),
                Color32::from_rgb(163, 210, 183),
                Color32::from_rgb(163, 190, 140), // Nord 14
                Color32::from_rgb(143, 188, 187), // Nord 7
                Color32::from_rgb(120, 175, 160),
                Color32::from_rgb(100, 160, 140),
                Color32::from_rgb(80, 145, 120),
                Color32::from_rgb(46, 52, 64),
            ],
            // Nord Aurora (Red)
            red: [
                Color32::from_rgb(250, 230, 230),
                Color32::from_rgb(245, 210, 210),
                Color32::from_rgb(240, 190, 190),
                Color32::from_rgb(235, 170, 170),
                Color32::from_rgb(208, 135, 112), // Nord 12
                Color32::from_rgb(191, 97, 106),  // Nord 11
                Color32::from_rgb(180, 85, 95),
                Color32::from_rgb(170, 75, 85),
                Color32::from_rgb(160, 65, 75),
                Color32::from_rgb(46, 52, 64),
            ],
            // Nord Aurora (Orange/Yellow)
            yellow: [
                Color32::from_rgb(254, 245, 220),
                Color32::from_rgb(252, 235, 190),
                Color32::from_rgb(250, 225, 160),
                Color32::from_rgb(248, 215, 130),
                Color32::from_rgb(235, 203, 139), // Nord 13
                Color32::from_rgb(215, 180, 115),
                Color32::from_rgb(195, 160, 95),
                Color32::from_rgb(175, 140, 75),
                Color32::from_rgb(155, 120, 55),
                Color32::from_rgb(46, 52, 64),
            ],
            // Nord Aurora (Purple)
            purple: [
                Color32::from_rgb(235, 225, 240),
                Color32::from_rgb(220, 205, 230),
                Color32::from_rgb(205, 185, 220),
                Color32::from_rgb(190, 165, 210),
                Color32::from_rgb(180, 142, 173), // Nord 15
                Color32::from_rgb(165, 130, 160),
                Color32::from_rgb(150, 115, 145),
                Color32::from_rgb(135, 100, 130),
                Color32::from_rgb(120, 85, 115),
                Color32::from_rgb(46, 52, 64),
            ],
            // Nord Frost (Teal variant)
            teal: [
                Color32::from_rgb(220, 238, 242),
                Color32::from_rgb(200, 226, 233),
                Color32::from_rgb(180, 214, 224),
                Color32::from_rgb(160, 202, 215),
                Color32::from_rgb(136, 192, 208), // Nord 8
                Color32::from_rgb(118, 180, 195),
                Color32::from_rgb(100, 168, 182),
                Color32::from_rgb(82, 156, 169),
                Color32::from_rgb(64, 144, 156),
                Color32::from_rgb(46, 52, 64),
            ],
        }
    }

    /// Dracula theme palette (popular dark theme)
    pub fn dracula() -> Self {
        Self {
            // Dracula Background + Foreground shades
            gray: [
                Color32::from_rgb(248, 248, 242), // Foreground
                Color32::from_rgb(241, 250, 238), // Light variant
                Color32::from_rgb(230, 230, 230),
                Color32::from_rgb(191, 191, 191),
                Color32::from_rgb(138, 138, 138),
                Color32::from_rgb(98, 114, 164),  // Comment
                Color32::from_rgb(68, 71, 90),    // Current Line
                Color32::from_rgb(48, 51, 70),    // Selection
                Color32::from_rgb(40, 42, 54),    // Background
                Color32::from_rgb(30, 32, 44),    // Darker background
            ],
            // Dracula Cyan + Blue
            blue: [
                Color32::from_rgb(240, 253, 255),
                Color32::from_rgb(224, 249, 255),
                Color32::from_rgb(208, 245, 255),
                Color32::from_rgb(192, 241, 255),
                Color32::from_rgb(139, 233, 253), // Cyan
                Color32::from_rgb(120, 220, 240),
                Color32::from_rgb(100, 207, 227),
                Color32::from_rgb(80, 194, 214),
                Color32::from_rgb(60, 181, 201),
                Color32::from_rgb(40, 42, 54),
            ],
            // Dracula Green
            green: [
                Color32::from_rgb(245, 255, 245),
                Color32::from_rgb(230, 255, 230),
                Color32::from_rgb(215, 255, 215),
                Color32::from_rgb(200, 255, 200),
                Color32::from_rgb(80, 250, 123), // Green
                Color32::from_rgb(70, 235, 113),
                Color32::from_rgb(60, 220, 103),
                Color32::from_rgb(50, 205, 93),
                Color32::from_rgb(40, 190, 83),
                Color32::from_rgb(40, 42, 54),
            ],
            // Dracula Red + Pink
            red: [
                Color32::from_rgb(255, 245, 250),
                Color32::from_rgb(255, 230, 240),
                Color32::from_rgb(255, 215, 230),
                Color32::from_rgb(255, 200, 220),
                Color32::from_rgb(255, 121, 198), // Pink
                Color32::from_rgb(255, 85, 85),   // Red
                Color32::from_rgb(240, 70, 70),
                Color32::from_rgb(225, 55, 55),
                Color32::from_rgb(210, 40, 40),
                Color32::from_rgb(40, 42, 54),
            ],
            // Dracula Yellow + Orange
            yellow: [
                Color32::from_rgb(255, 252, 235),
                Color32::from_rgb(255, 248, 220),
                Color32::from_rgb(255, 244, 205),
                Color32::from_rgb(255, 240, 190),
                Color32::from_rgb(241, 250, 140), // Yellow
                Color32::from_rgb(255, 184, 108), // Orange
                Color32::from_rgb(245, 170, 95),
                Color32::from_rgb(235, 156, 82),
                Color32::from_rgb(225, 142, 69),
                Color32::from_rgb(40, 42, 54),
            ],
            // Dracula Purple
            purple: [
                Color32::from_rgb(250, 245, 255),
                Color32::from_rgb(240, 230, 255),
                Color32::from_rgb(230, 215, 255),
                Color32::from_rgb(220, 200, 255),
                Color32::from_rgb(189, 147, 249), // Purple
                Color32::from_rgb(175, 130, 235),
                Color32::from_rgb(161, 113, 221),
                Color32::from_rgb(147, 96, 207),
                Color32::from_rgb(133, 79, 193),
                Color32::from_rgb(40, 42, 54),
            ],
            // Dracula Cyan (same as blue but different shades)
            teal: [
                Color32::from_rgb(240, 253, 255),
                Color32::from_rgb(224, 249, 255),
                Color32::from_rgb(208, 245, 255),
                Color32::from_rgb(192, 241, 255),
                Color32::from_rgb(139, 233, 253), // Cyan
                Color32::from_rgb(120, 220, 240),
                Color32::from_rgb(100, 207, 227),
                Color32::from_rgb(80, 194, 214),
                Color32::from_rgb(60, 181, 201),
                Color32::from_rgb(40, 42, 54),
            ],
        }
    }
}
