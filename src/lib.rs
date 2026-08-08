//! Cleanroom Rust port of upstream Go source file: `ansi/doc.go`
//! Upstream Target Tag / Version: `v0.11.2`
//!
//! <public-docs>
//! An ANSI escape-code library for Go, ported to Rust. Provides SGR styles,
//! string width measurement, wrapping, truncation, hyperlinks, and color
//! conversion.
//! </public-docs>

#![deny(unsafe_code)]

pub mod color;
pub mod hyperlink;
pub mod method;
pub mod style;
pub mod util;
pub mod width;
pub mod wrap;

pub use color::{convert_16, convert_256, ansi256_to_16, BasicColor, IndexedColor, RGBColor};
pub use hyperlink::{reset_hyperlink, set_hyperlink};
pub use method::WidthMethod;
pub use style::{Color, Style, Underline, RESET_STYLE};
pub use util::{cut, cut_left, strip, string_width, truncate, truncate_left};
pub use width::first_grapheme_cluster;
pub use wrap::{hardwrap, hardwrap_wc, wrap, wrap_wc};
