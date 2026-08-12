//! Cleanroom Rust port of upstream Go source file: `ansi/method.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! Width method abstraction for measuring graphemes.
//! </public-docs>

use crate::width;
use std::sync::OnceLock;

/// WidthMethod is a method used to measure the width of a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WidthMethod {
    /// Measure the width of each grapheme cluster.
    GraphemeWidth,
    /// Measure the width of each wide character and rune.
    #[default]
    WcWidth,
}

impl WidthMethod {
    /// Returns the width of the given string in cells.
    pub fn string_width(&self, s: &str) -> usize {
        match self {
            WidthMethod::WcWidth => width::string_width_wc(s),
            WidthMethod::GraphemeWidth => width::string_width(s),
        }
    }
}

/// Whether the `RUNEWIDTH_EASTASIAN` environment variable enables treating
/// ambiguous East Asian characters as double-width, mirroring the upstream
/// `init` in `ansi/method.go`.
pub(crate) fn east_asian_width() -> bool {
    static EAST_ASIAN: OnceLock<bool> = OnceLock::new();
    *EAST_ASIAN.get_or_init(|| match std::env::var("RUNEWIDTH_EASTASIAN") {
        Ok(v) => parse_bool(&v).unwrap_or(false),
        Err(_) => false,
    })
}

/// Mirrors Go's `strconv.ParseBool`: accepts 1, t, T, TRUE, true, True, 0, f,
/// F, FALSE, false, False.
fn parse_bool(v: &str) -> Option<bool> {
    match v {
        "1" | "t" | "T" | "TRUE" | "true" | "True" => Some(true),
        "0" | "f" | "F" | "FALSE" | "false" | "False" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_string_width() {
        let cases = [
            ("empty string wcwidth", WidthMethod::WcWidth, "", 0),
            (
                "empty string grapheme width",
                WidthMethod::GraphemeWidth,
                "",
                0,
            ),
            ("ascii wcwidth", WidthMethod::WcWidth, "hello", 5),
            (
                "ascii grapheme width",
                WidthMethod::GraphemeWidth,
                "hello",
                5,
            ),
            (
                "ansi wcwidth",
                WidthMethod::WcWidth,
                "\x1b[31mred\x1b[0m",
                3,
            ),
            (
                "ansi grapheme width",
                WidthMethod::GraphemeWidth,
                "\x1b[31mred\x1b[0m",
                3,
            ),
            ("wide chars wcwidth", WidthMethod::WcWidth, "コンニチハ", 10),
            (
                "wide chars grapheme width",
                WidthMethod::GraphemeWidth,
                "コンニチハ",
                10,
            ),
            ("emoji wcwidth", WidthMethod::WcWidth, "😀", 2),
            ("emoji grapheme width", WidthMethod::GraphemeWidth, "😀", 2),
            (
                "flag emoji wcwidth",
                WidthMethod::WcWidth,
                "🏳\u{fe0f}\u{200d}🌈",
                1,
            ),
            (
                "flag emoji grapheme width",
                WidthMethod::GraphemeWidth,
                "🏳\u{fe0f}\u{200d}🌈",
                2,
            ),
        ];
        for (name, m, input, want) in cases {
            let got = m.string_width(input);
            assert_eq!(
                got, want,
                "{name}: Method.StringWidth({input:?}) = {got}, want {want}"
            );
        }
    }
}
