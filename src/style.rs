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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Underline {
    /// No underline.
    #[default]
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
            _ if has_more(&s)
                && has_more(&p)
                && unpack(p, 0) == 2
                && has_more(&params[2])
                && has_more(&params[3])
                && !has_more(&params[4]) =>
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
            _ if !has_more(&s)
                && !has_more(&p)
                && unpack(p, 0) == 2
                && !has_more(&params[2])
                && !has_more(&params[3])
                && !has_more(&params[4]) =>
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
        5 => {
            // indexed color
            if params.len() < 3 {
                return 0;
            }
            match () {
                _ if has_more(&s) && has_more(&p) && !has_more(&params[2]) => {
                    // Colon separated indexed color
                    // 38 : 5 : 234
                }
                _ if !has_more(&s) && !has_more(&p) && !has_more(&params[2]) => {
                    // Legacy semicolon indexed color
                    // 38 ; 5 ; 234
                }
                _ => return 0,
            }
            *co = Some(Color::Indexed(unpack(params[2], 0) as u8));
            3
        }
        6 => {
            // RGBA direct color
            if params.len() < 6 {
                return 0;
            }

            let mut n2 = n;
            let (r, g, b, a) = paramsfn(params, &mut n2);
            if r == -1 || g == -1 || b == -1 || a == -1 {
                return 0;
            }
            n = n2;

            *co = Some(Color::RGB(crate::color::RGBColor {
                r: r as u8,
                g: g as u8,
                b: b as u8,
            }));
            // NOTE: upstream stores color.RGBA including alpha; the ported
            // RGBColor has no alpha channel, so it is dropped.
            let _ = a;
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

/// Returns the SGR parameter for the given color and base (30/40/50).
pub fn color_seq(c: &Color, base: u8) -> String {
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
    use crate::parser::{parameter, Param};

    #[test]
    fn test_sgr_sequences() {
        let s = Style {
            bold: true,
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[1m");
        assert_eq!(s.styled("hello"), "\x1b[1mhello\x1b[m");

        let s = Style {
            fg_color: Some(Color::RGB(RGBColor {
                r: 90,
                g: 86,
                b: 224,
            })),
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[38;2;90;86;224m");
        assert_eq!(s.styled("hello"), "\x1b[38;2;90;86;224mhello\x1b[m");
    }

    #[test]
    fn test_underline_styles() {
        let s = Style {
            underline: true,
            underline_style: Underline::Single,
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[4;4m");

        let s = Style {
            underline: true,
            underline_style: Underline::Curly,
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[4;4:3m");

        let s = Style {
            underline: true,
            underline_style: Underline::Curly,
            ul_color: Some(Color::RGB(RGBColor { r: 255, g: 0, b: 0 })),
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[4;58;2;255;0;0;4:3m");
    }

    #[test]
    fn test_nil_colors() {
        let s = Style {
            fg_color: Some(Color::Default),
            bg_color: Some(Color::Default),
            ul_color: Some(Color::Default),
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[39;49;59m");
    }

    #[test]
    fn test_color_seq_variants() {
        // Basic colors: dim and bright.
        assert_eq!(color_seq(&Color::Basic(1), 3), "31");
        assert_eq!(color_seq(&Color::Basic(9), 3), "91");
        assert_eq!(color_seq(&Color::Basic(8), 4), "100");
        assert_eq!(color_seq(&Color::Basic(15), 4), "107");
        // Default colors.
        assert_eq!(color_seq(&Color::Default, 3), "39");
        assert_eq!(color_seq(&Color::Default, 5), "59");
        // Indexed colors.
        assert_eq!(color_seq(&Color::Indexed(196), 3), "38;5;196");
        assert_eq!(color_seq(&Color::Indexed(42), 4), "48;5;42");
        // RGB colors.
        assert_eq!(
            color_seq(&Color::RGB(RGBColor { r: 1, g: 2, b: 3 }), 3),
            "38;2;1;2;3"
        );
        assert_eq!(
            color_seq(&Color::RGB(RGBColor { r: 1, g: 2, b: 3 }), 5),
            "58;2;1;2;3"
        );
    }

    #[test]
    fn test_string_and_styled_edge_cases() {
        // Empty style renders nothing and styled returns the input unchanged.
        let s = Style::default();
        assert_eq!(s.string(), "");
        assert_eq!(s.styled("hello"), "hello");
        // All attributes together.
        let s = Style {
            bold: true,
            faint: true,
            italic: true,
            underline: true,
            blink: true,
            reverse: true,
            strikethrough: true,
            fg_color: Some(Color::Indexed(196)),
            bg_color: Some(Color::RGB(RGBColor { r: 0, g: 0, b: 0 })),
            ul_color: Some(Color::Basic(1)),
            underline_style: Underline::Double,
        };
        assert_eq!(s.string(), "\x1b[1;2;3;4;5;7;9;38;5;196;48;2;0;0;0;51;21m");
        // Curly underline with no explicit color.
        let s = Style {
            underline: true,
            underline_style: Underline::Curly,
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[4;4:3m");
        // Dotted/dashed underline styles.
        let s = Style {
            underline: true,
            underline_style: Underline::Dotted,
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[4;4:4m");
        let s = Style {
            underline: true,
            underline_style: Underline::Dashed,
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[4;4:5m");
        // Underline::None with underline set emits just the base 4.
        let s = Style {
            underline: true,
            underline_style: Underline::None,
            ..Default::default()
        };
        assert_eq!(s.string(), "\x1b[4m");
        // RGBColor hex formatting.
        assert_eq!(
            RGBColor {
                r: 0xff,
                g: 0x00,
                b: 0xff
            }
            .hex(),
            "#ff00ff"
        );
        assert_eq!(RGBColor { r: 1, g: 2, b: 3 }.hex(), "#010203");
    }

    /// Legacy semicolon-separated RGB: 38;2;r;g;b.
    #[test]
    fn test_read_style_color_rgb_semicolon() {
        let params = [38, 2, 10, 20, 30];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 5);
        assert_eq!(
            co,
            Some(Color::RGB(RGBColor {
                r: 10,
                g: 20,
                b: 30
            }))
        );
    }

    /// Colon-separated RGB: 38:2:r:g:b.
    #[test]
    fn test_read_style_color_rgb_colon() {
        let params = [
            parameter(38, true),
            parameter(2, true),
            parameter(10, true),
            parameter(20, true),
            parameter(30, false),
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 5);
        assert_eq!(
            co,
            Some(Color::RGB(RGBColor {
                r: 10,
                g: 20,
                b: 30
            }))
        );
    }

    /// RGB with color space id and tolerance (colon): 38:2:cs:r:g:b:t:tcs.
    #[test]
    fn test_read_style_color_rgb_colorspace() {
        let params = [
            parameter(38, true),
            parameter(2, true),
            parameter(1, true),  // color space id
            parameter(10, true), // r
            parameter(20, true), // g
            parameter(30, true), // b
            parameter(5, false), // tolerance
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 7);
        assert_eq!(
            co,
            Some(Color::RGB(RGBColor {
                r: 10,
                g: 20,
                b: 30
            }))
        );
    }

    /// RGB with color space id, tolerance, and tolerance color space (8 params).
    #[test]
    fn test_read_style_color_rgb_colorspace_tol_cs() {
        let params = [
            parameter(38, true),
            parameter(2, true),
            parameter(1, true),  // color space id
            parameter(10, true), // r
            parameter(20, true), // g
            parameter(30, true), // b
            parameter(5, true),  // tolerance
            parameter(7, true),  // tolerance color space
            parameter(0, false),
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 9);
        assert_eq!(
            co,
            Some(Color::RGB(RGBColor {
                r: 10,
                g: 20,
                b: 30
            }))
        );
    }

    /// Indexed color, semicolon: 38;5;234.
    #[test]
    fn test_read_style_color_indexed_semicolon() {
        let params = [38, 5, 234];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 3);
        assert_eq!(co, Some(Color::Indexed(234)));
    }

    /// Indexed color, colon: 38:5:234.
    #[test]
    fn test_read_style_color_indexed_colon() {
        let params = [
            parameter(38, true),
            parameter(5, true),
            parameter(234, false),
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 3);
        assert_eq!(co, Some(Color::Indexed(234)));
    }

    /// CMY direct color: 38:3:cs:c:m:y.
    #[test]
    fn test_read_style_color_cmy() {
        let params = [
            parameter(38, true),
            parameter(3, true),
            parameter(1, true),
            parameter(10, true),
            parameter(20, true),
            parameter(30, false),
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 6);
        assert_eq!(
            co,
            Some(Color::RGB(RGBColor {
                r: 245,
                g: 235,
                b: 225
            }))
        );
    }

    /// CMYK direct color: 38:4:cs:c:m:y:k.
    #[test]
    fn test_read_style_color_cmyk() {
        let params = [
            parameter(38, true),
            parameter(4, true),
            parameter(1, true),
            parameter(0, true),
            parameter(0, true),
            parameter(0, true),
            parameter(255, false),
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 7);
        assert_eq!(co, Some(Color::RGB(RGBColor { r: 0, g: 0, b: 0 })));
    }

    /// RGBA direct color: 38:6:cs:r:g:b:a.
    #[test]
    fn test_read_style_color_rgba() {
        let params = [
            parameter(38, true),
            parameter(6, true),
            parameter(1, true),
            parameter(10, true),
            parameter(20, true),
            parameter(30, true),
            parameter(255, false),
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 7);
        assert_eq!(
            co,
            Some(Color::RGB(RGBColor {
                r: 10,
                g: 20,
                b: 30
            }))
        );
    }

    /// Implementation-defined and transparent color types.
    #[test]
    fn test_read_style_color_defined_transparent() {
        let params = [38, 0, 1];
        let mut co = Some(Color::Indexed(5));
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 2);
        assert_eq!(co, None);

        let params = [38, 1, 1];
        let mut co = Some(Color::Indexed(5));
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 2);
        assert_eq!(co, None);
    }

    /// Too few params, unknown types, and ambiguous colors return 0.
    #[test]
    fn test_read_style_color_errors() {
        let mut co = None;
        // Fewer than 2 params.
        assert_eq!(read_style_color(&[38], &mut co), 0);
        // Unknown color type.
        assert_eq!(read_style_color(&[38, 9, 1, 2, 3], &mut co), 0);
        // RGB with too few params.
        assert_eq!(read_style_color(&[38, 2, 1], &mut co), 0);
        // RGB with a missing channel.
        let params = [38, 2, 10, 20, i32::MIN];
        assert_eq!(read_style_color(&params, &mut co), 0);
        // CMY with too few params.
        assert_eq!(read_style_color(&[38, 3, 1, 2], &mut co), 0);
        // CMY with a missing channel.
        let params = [38, 3, 10, 20, i32::MIN];
        assert_eq!(read_style_color(&params, &mut co), 0);
        // CMYK with too few params.
        assert_eq!(read_style_color(&[38, 4, 1, 2, 3], &mut co), 0);
        // CMYK with a missing channel.
        let params = [38, 4, 10, 20, 30, i32::MIN];
        assert_eq!(read_style_color(&params, &mut co), 0);
        // Indexed with too few params.
        assert_eq!(read_style_color(&[38, 5], &mut co), 0);
        // Indexed with an inconsistent separator.
        let params = [parameter(38, true), parameter(5, false), 234];
        assert_eq!(read_style_color(&params, &mut co), 0);
        // RGBA with too few params.
        assert_eq!(read_style_color(&[38, 6, 1, 2, 3], &mut co), 0);
        // RGBA with a missing channel.
        let params = [38, 6, 10, 20, 30, i32::MIN];
        assert_eq!(read_style_color(&params, &mut co), 0);
        // Missing param defaults to 0.
        let params = [38, 2, 10, 20, Param(i32::MAX).0];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 5);
        assert_eq!(co, Some(Color::RGB(RGBColor { r: 10, g: 20, b: 0 })));
        // Tolerance + tolerance color space (8-param colon form).
        let params = [
            parameter(38, true),
            parameter(2, true),
            parameter(1, true),
            parameter(10, true),
            parameter(20, true),
            parameter(30, true),
            parameter(5, true),
            parameter(7, true),
            parameter(0, false),
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 9);
        assert_eq!(
            co,
            Some(Color::RGB(RGBColor {
                r: 10,
                g: 20,
                b: 30
            }))
        );
        // Tolerance without tolerance color space (7-param colon form).
        let params = [
            parameter(38, true),
            parameter(2, true),
            parameter(1, true),
            parameter(10, true),
            parameter(20, true),
            parameter(30, true),
            parameter(5, true),
            parameter(0, false),
        ];
        let mut co = None;
        let n = read_style_color(&params, &mut co);
        assert_eq!(n, 8);
        assert_eq!(
            co,
            Some(Color::RGB(RGBColor {
                r: 10,
                g: 20,
                b: 30
            }))
        );
    }
}
