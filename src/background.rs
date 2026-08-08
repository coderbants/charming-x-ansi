//! Cleanroom Rust port of upstream Go source file: `ansi/background.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! OSC 10/11/12 sequences for querying and setting the default terminal
//! foreground, background, and cursor colors.
//! </public-docs>

/// HexColor is a color that can be formatted as a hex string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexColor(pub &'static str);

impl HexColor {
    /// Returns the hex representation of the color.
    pub fn hex(&self) -> String {
        match crate::util::x_parse_color(self.0) {
            Some(c) => format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b),
            None => String::new(),
        }
    }
}

/// XRGBColor is a color that can be formatted as an XParseColor `rgb:` string.
///
/// See: <https://linux.die.net/man/3/xparsecolor>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XRGBColor(pub crate::color::RGBColor);

impl XRGBColor {
    /// Returns the color as an XParseColor `rgb:` string.
    pub fn string(&self) -> String {
        format!(
            "rgb:{:04x}/{:04x}/{:04x}",
            self.0.r as u16 * 257,
            self.0.g as u16 * 257,
            self.0.b as u16 * 257
        )
    }
}

/// XRGBAColor is a color that can be formatted as an XParseColor `rgba:`
/// string.
///
/// See: <https://linux.die.net/man/3/xparsecolor>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XRGBAColor(pub crate::color::RGBColor);

impl XRGBAColor {
    /// Returns the color as an XParseColor `rgba:` string.
    pub fn string(&self) -> String {
        format!(
            "rgba:{:04x}/{:04x}/{:04x}/ffff",
            self.0.r as u16 * 257,
            self.0.g as u16 * 257,
            self.0.b as u16 * 257
        )
    }
}

/// Returns a sequence that sets the default terminal foreground color.
///
/// `OSC 10 ; color ST` / `OSC 10 ; color BEL`
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub fn set_foreground_color(s: &str) -> String {
    format!("\x1b]10;{}\x07", s)
}

/// Requests the current default terminal foreground color.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub const REQUEST_FOREGROUND_COLOR: &str = "\x1b]10;?\x07";

/// Resets the default terminal foreground color.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub const RESET_FOREGROUND_COLOR: &str = "\x1b]110\x07";

/// Returns a sequence that sets the default terminal background color.
///
/// `OSC 11 ; color ST` / `OSC 11 ; color BEL`
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub fn set_background_color(s: &str) -> String {
    format!("\x1b]11;{}\x07", s)
}

/// Requests the current default terminal background color.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub const REQUEST_BACKGROUND_COLOR: &str = "\x1b]11;?\x07";

/// Resets the default terminal background color.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub const RESET_BACKGROUND_COLOR: &str = "\x1b]111\x07";

/// Returns a sequence that sets the terminal cursor color.
///
/// `OSC 12 ; color ST` / `OSC 12 ; color BEL`
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub fn set_cursor_color(s: &str) -> String {
    format!("\x1b]12;{}\x07", s)
}

/// Requests the current terminal cursor color.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub const REQUEST_CURSOR_COLOR: &str = "\x1b]12;?\x07";

/// Resets the terminal cursor color.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands>
pub const RESET_CURSOR_COLOR: &str = "\x1b]112\x07";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_foreground_color() {
        assert_eq!(set_foreground_color("#ff00ff"), "\x1b]10;#ff00ff\x07");
        assert_eq!(REQUEST_FOREGROUND_COLOR, "\x1b]10;?\x07");
        assert_eq!(RESET_FOREGROUND_COLOR, "\x1b]110\x07");
    }

    #[test]
    fn test_set_background_color() {
        assert_eq!(set_background_color("#ffffff"), "\x1b]11;#ffffff\x07");
        assert_eq!(REQUEST_BACKGROUND_COLOR, "\x1b]11;?\x07");
        assert_eq!(RESET_BACKGROUND_COLOR, "\x1b]111\x07");
    }

    #[test]
    fn test_set_cursor_color() {
        assert_eq!(set_cursor_color("#000000"), "\x1b]12;#000000\x07");
        assert_eq!(REQUEST_CURSOR_COLOR, "\x1b]12;?\x07");
        assert_eq!(RESET_CURSOR_COLOR, "\x1b]112\x07");
    }

    #[test]
    fn test_hex_color() {
        let h = HexColor("#ff00ff");
        assert_eq!(h.hex(), "#ff00ff");
        let h = HexColor("#abc");
        assert_eq!(h.hex(), "#aabbcc");
        let h = HexColor("not-a-color");
        assert_eq!(h.hex(), "");
    }

    #[test]
    fn test_xrgb_color() {
        let c = XRGBColor(crate::color::RGBColor { r: 255, g: 0, b: 0 });
        assert_eq!(c.string(), "rgb:ffff/0000/0000");
        let c = XRGBAColor(crate::color::RGBColor { r: 1, g: 2, b: 3 });
        assert_eq!(c.string(), "rgba:0101/0202/0303/ffff");
    }
}
