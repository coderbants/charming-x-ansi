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

/// ReadStyleColor reads an SGR color (38/48/58 extended color sequence) from
/// the given parameters starting at index 0, returning the number of
/// parameters consumed and the color.
///
/// NOTE: upstream returns Go `color.Color` values (RGB, CMYK); the port maps
/// CMYK to RGB via the standard conversion and returns `Option<Color>`.
pub fn read_style_color(params: &[i32], co: &mut Option<Color>) -> usize {
    use crate::parser::{HAS_MORE_FLAG, MISSING_PARAM};

    let has_more = |p: &i32| p & HAS_MORE_FLAG != 0;
    let unpack = |p: i32, def: i32| -> i32 {
        let v = p & !HAS_MORE_FLAG;
        if v == MISSING_PARAM {
            def
        } else {
            v
        }
    };

    if params.len() < 2 {
        // Need at least SGR type and color type
        return 0;
    }

    // First parameter indicates one of 38, 48, or 58.
    let s = params[0];
    let p = params[1];
    let color_type = unpack(p, 0);
    let mut n = 2;

    let paramsfn = |params: &[i32], n: &mut usize| -> (i32, i32, i32, i32) {
        // Where should we start reading the color?
        match () {
            _ if has_more(&s)
                && has_more(&p)
                && params.len() > 8
                && has_more(&params[2])
                && has_more(&params[3])
                && has_more(&params[4])
                && has_more(&params[5])
                && has_more(&params[6])
                && has_more(&params[7]) =>
            {
                // We have color space id, a 6th parameter, a tolerance value,
                // and a tolerance color space
                *n += 7;
                (
                    unpack(params[3], 0),
                    unpack(params[4], 0),
                    unpack(params[5], 0),
                    unpack(params[6], 0),
                )
            }
            _ if has_more(&s)
                && has_more(&p)
                && params.len() > 7
                && has_more(&params[2])
                && has_more(&params[3])
                && has_more(&params[4])
                && has_more(&params[5])
                && has_more(&params[6]) =>
            {
                // We have color space id, a 6th parameter, and a tolerance
                // value
                *n += 6;
                (
                    unpack(params[3], 0),
                    unpack(params[4], 0),
                    unpack(params[5], 0),
                    unpack(params[6], 0),
                )
            }
            _ if has_more(&s)
                && has_more(&p)
                && params.len() > 6
                && has_more(&params[2])
                && has_more(&params[3])
                && has_more(&params[4])
                && has_more(&params[5]) =>
            {
                // We have color space id and a 6th parameter
                *n += 5;
                (
                    unpack(params[3], 0),
                    unpack(params[4], 0),
                    unpack(params[5], 0),
                    unpack(params[6], 0),
                )
            }
            _ if has_more(&s)
                && has_more(&p)
                && params.len() > 5
                && has_more(&params[2])
                && has_more(&params[3])
                && has_more(&params[4])
                && !has_more(&params[5]) =>
            {
                // We have color space
                *n += 4;
                (
                    unpack(params[3], 0),
                    unpack(params[4], 0),
                    unpack(params[5], 0),
                    -1,
                )
            }
            _ if has_more(&s) && has_more(&p) && unpack(p, 0) == 2
                && has_more(&params[2]) && has_more(&params[3]) && !has_more(&params[4]) =>
            {
                // We have color values separated by colons (:)
                *n += 3;
                (
                    unpack(params[2], 0),
                    unpack(params[3], 0),
                    unpack(params[4], 0),
                    -1,
                )
            }
            _ if !has_more(&s) && !has_more(&p) && unpack(p, 0) == 2
                && !has_more(&params[2]) && !has_more(&params[3]) && !has_more(&params[4]) =>
            {
                // Support legacy color values separated by semicolons (;)
                *n += 3;
                (
                    unpack(params[2], 0),
                    unpack(params[3], 0),
                    unpack(params[4], 0),
                    -1,
                )
            }
            _ => {
                // Ambiguous SGR color
                (-1, -1, -1, -1)
            }
        }
    };

    match color_type {
        0 => {
            // implementation defined
            *co = None;
            2
        }
        1 => {
            // transparent
            *co = None;
            2
        }
        2 => {
            // RGB direct color
            if params.len() < 5 {
                return 0;
            }

            let mut n2 = n;
            let (r, g, b, _) = paramsfn(params, &mut n2);
            if r == -1 || g == -1 || b == -1 {
                return 0;
            }
            n = n2;

            *co = Some(Color::RGB(crate::color::RGBColor {
                r: r as u8,
                g: g as u8,
                b: b as u8,
            }));
            n
        }
        3 => {
            // CMY direct color
            if params.len() < 5 {
                return 0;
            }

            let mut n2 = n;
            let (c, m, y, _) = paramsfn(params, &mut n2);
            if c == -1 || m == -1 || y == -1 {
                return 0;
            }
            n = n2;

            // NOTE: upstream stores color.CMYK; converted to RGB here.
            *co = Some(Color::RGB(crate::color::RGBColor {
                r: 255 - c as u8,
                g: 255 - m as u8,
                b: 255 - y as u8,
            }));
            n
        }
        4 => {
            // CMYK direct color
            if params.len() < 6 {
                return 0;
            }

            let mut n2 = n;
            let (c, m, y, k) = paramsfn(params, &mut n2);
            if c == -1 || m == -1 || y == -1 || k == -1 {
                return 0;
            }
            n = n2;

            // NOTE: upstream stores color.CMYK; converted to RGB here.
            let (c, m, y, k) = (
                c as f32 / 255.0,
                m as f32 / 255.0,
                y as f32 / 255.0,
                k as f32 / 255.0,
            );
            let conv = |v: f32| (255.0 * (1.0 - v) * (1.0 - k)).round() as u8;
            *co = Some(Color::RGB(crate::color::RGBColor {
                r: conv(c),
                g: conv(m),
                b: conv(y),
            }));
            n
        }
        _ => 0,
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
