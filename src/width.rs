//! Cleanroom Rust port of upstream Go source file: `ansi/width.go`
//! Upstream Target Tag / Version: `v0.11.2`
//!
//! <public-docs>
//! Width measurement helpers.
//! </public-docs>

/// Returns the width of the first grapheme cluster of the string.
pub fn first_grapheme_cluster(s: &str) -> Option<&str> {
    unicode_segmentation::UnicodeSegmentation::graphemes(s, true).next()
}
