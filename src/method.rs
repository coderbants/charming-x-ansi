//! Cleanroom Rust port of upstream Go source file: `ansi/method.go`
//! Upstream Target Tag / Version: `v0.11.2`
//!
//! <public-docs>
//! Width method abstraction for measuring graphemes.
//! </public-docs>

use crate::util::string_width;

/// WidthMethod is a method used to measure the width of a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthMethod {
    /// Measure the width of each grapheme cluster.
    GraphemeWidth,
    /// Measure the width of each wide character and rune.
    WcWidth,
}

impl Default for WidthMethod {
    fn default() -> Self {
        WidthMethod::WcWidth
    }
}

impl WidthMethod {
    /// Returns the width of the given string in cells.
    pub fn string_width(&self, s: &str) -> usize {
        string_width(s)
    }
}
