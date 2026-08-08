//! Cleanroom Rust port of upstream Go source file: `ansi/style.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! An ANSI SGR style builder. Output sequences match upstream byte-for-byte.
//! </public-docs>

use crate::color::{BasicColor, IndexedColor, RGBColor};

/// ResetStyle is a SGR (Select Graphic Rendition) style sequence that resets
/// all attributes.
/// See: <https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters>
pub const RESET_STYLE: &str = "\x1b[m";

/// Style of the underline.
///
/// Caveats:
/// - Not all terminals support all underline styles.
/// - Some terminals may render unsupported styles as standard underlines.
/// - Terminal themes may affect the visibility of different underline styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Underline {
    /// No underline.
    None,
    /// A single underline. This is the default when underline is enabled.
    Single,
    /// A double underline.
    Double,
    /// A curly underline.
    Curly,
    /// A dotted underline.
    Dotted,
    /// A dashed underline.
    Dashed,
}

impl Default for Underline {
    fn default() -> Self {
        Underline::None
    }
}

/// An ANSI SGR style. A zero value renders nothing.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    /// Bold text attribute (SGR 1).
    pub bold: bool,
    /// Italic text attribute (SGR 3).
    pub italic: bool,
    /// Underline text attribute (SGR 4).
    pub underline: bool,
    /// The style of the underline, if underlined.
    pub underline_style: Underline,
    /// Reverse video attribute (SGR 7).
    pub reverse: bool,
    /// Blink attribute (SGR 5).
    pub blink: bool,
    /// Faint/dim attribute (SGR 2).
    pub faint: bool,
    /// Strikethrough attribute (SGR 9).
    pub strikethrough: bool,
    /// Foreground color.
    pub fg_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Underline color.
    pub ul_color: Option<Color>,
}

/// A terminal color: the default color, a 4-bit basic color, an 8-bit indexed
/// color, or a 24-bit RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// The terminal's default color (SGR 39/49/59). Mirrors a nil color in
    /// upstream, which renders the terminal default foreground, background, or
    /// underline color.
    Default,
    /// 16-color ANSI code (0-15).
    Basic(BasicColor),
    /// 256-color ANSI code (0-255).
    Indexed(IndexedColor),
    /// 24-bit TrueColor RGB value.
    RGB(RGBColor),
}

impl Style {
    /// Returns whether this style has no attributes or colors set.
    pub fn is_zero(&self) -> bool {
        self == &Style::default()
    }

    /// Returns the SGR sequence for this style, e.g. `\x1b[1;38;2;90;86;224m`.
    pub fn string(&self) -> String {
        let mut params: Vec<String> = Vec::new();
        if self.bold {
            params.push("1".to_string());
        }
        if self.faint {
            params.push("2".to_string());
        }
        if self.italic {
            params.push("3".to_string());
        }
        if self.underline {
            params.push("4".to_string());
        }
        if self.blink {
            params.push("5".to_string());
        }
        if self.reverse {
            params.push("7".to_string());
        }
        if self.strikethrough {
            params.push("9".to_string());
        }
        if let Some(c) = self.fg_color {
            params.push(color_seq(&c, 3));
        }
        if let Some(c) = self.bg_color {
            params.push(color_seq(&c, 4));
        }
        if let Some(c) = self.ul_color {
            params.push(color_seq(&c, 5));
        }
        if self.underline {
            match self.underline_style {
                Underline::None => {}
                Underline::Single => params.push("4".to_string()),
                Underline::Double => params.push("21".to_string()),
                Underline::Curly => params.push("4:3".to_string()),
                Underline::Dotted => params.push("4:4".to_string()),
                Underline::Dashed => params.push("4:5".to_string()),
            }
        }
        if params.is_empty() {
            return String::new();
        }
        format!("\x1b[{}m", params.join(";"))
    }

    /// Applies the style to the given string, wrapping it in the SGR sequence
    /// and an ANSI reset.
    pub fn styled(&self, s: &str) -> String {
        if self.is_zero() {
            return s.to_string();
        }
        format!("{}{}{}", self.string(), s, RESET_STYLE)
    }
}

fn color_seq(c: &Color, base: u8) -> String {
    match c {
        Color::Default => format!("{}9", base),
        Color::Basic(v) => {
            if *v < 8 {
                format!("{}", base * 10 + v)
            } else {
                format!("{}", base * 10 + 60 + (v - 8))
            }
        }
        Color::Indexed(v) => format!("{};5;{}", base * 10 + 8, v),
        Color::RGB(v) => format!("{};2;{};{};{}", base * 10 + 8, v.r, v.g, v.b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sgr_sequences() {
        let mut s = Style::default();
        s.bold = true;
        assert_eq!(s.string(), "\x1b[1m");
        assert_eq!(s.styled("hello"), "\x1b[1mhello\x1b[m");

        let mut s = Style::default();
        s.fg_color = Some(Color::RGB(RGBColor { r: 90, g: 86, b: 224 }));
        assert_eq!(s.string(), "\x1b[38;2;90;86;224m");
        assert_eq!(s.styled("hello"), "\x1b[38;2;90;86;224mhello\x1b[m");
    }

    #[test]
    fn test_underline_styles() {
        let mut s = Style::default();
        s.underline = true;
        s.underline_style = Underline::Single;
        assert_eq!(s.string(), "\x1b[4;4m");

        let mut s = Style::default();
        s.underline = true;
        s.underline_style = Underline::Curly;
        assert_eq!(s.string(), "\x1b[4;4:3m");

        let mut s = Style::default();
        s.underline = true;
        s.underline_style = Underline::Curly;
        s.ul_color = Some(Color::RGB(RGBColor { r: 255, g: 0, b: 0 }));
        assert_eq!(s.string(), "\x1b[4;58;2;255;0;0;4:3m");
    }

    #[test]
    fn test_nil_colors() {
        let mut s = Style::default();
        s.fg_color = Some(Color::Default);
        s.bg_color = Some(Color::Default);
        s.ul_color = Some(Color::Default);
        assert_eq!(s.string(), "\x1b[39;49;59m");
    }
}
