//! Cleanroom Rust port of upstream Go source file: `ansi/util.go` and `ansi/width.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! ANSI-aware string utilities: stripping, cutting, truncation, and cell-width
//! measurement.
//! </public-docs>

use unicode_width::UnicodeWidthChar;

use crate::color::RGBColor;

/// StringWidth returns the width of the string in cells, ignoring ANSI
/// sequences and measuring wide characters correctly.
pub fn string_width(s: &str) -> usize {
    crate::width::string_width(s)
}

/// Shift reduces a 16-bit component value to 8 bits, mirroring the upstream
/// `shift` helper.
fn shift(v: u64) -> u8 {
    if v > 0xff {
        (v >> 8) as u8
    } else {
        v as u8
    }
}

/// XParseColor is a helper function that parses a string into an RGB color.
/// It provides a similar interface to the XParseColor function in Xlib. It
/// supports the following formats:
///
/// - #RGB
/// - #RRGGBB
/// - rgb:RRRR/GGGG/BBBB
/// - rgba:RRRR/GGGG/BBBB/AAAA
///
/// If the string is not a valid color, `None` is returned.
///
/// See: <https://linux.die.net/man/3/xparsecolor>
pub fn x_parse_color(s: &str) -> Option<RGBColor> {
    if let Some(hex) = s.strip_prefix('#') {
        return match hex.len() {
            3 => {
                let nib = |i: usize| -> Option<u8> {
                    u8::from_str_radix(&hex[i..i + 1], 16).ok()
                };
                let r = nib(0)?.checked_mul(17)?;
                let g = nib(1)?.checked_mul(17)?;
                let b = nib(2)?.checked_mul(17)?;
                Some(RGBColor { r, g, b })
            }
            4 => {
                let nib = |i: usize| -> Option<u8> {
                    u8::from_str_radix(&hex[i..i + 1], 16).ok()
                };
                let r = nib(0)?.checked_mul(17)?;
                let g = nib(1)?.checked_mul(17)?;
                let b = nib(2)?.checked_mul(17)?;
                Some(RGBColor { r, g, b })
            }
            6 => {
                let pair = |i: usize| -> Option<u8> {
                    u8::from_str_radix(&hex[i..i + 2], 16).ok()
                };
                let r = pair(0)?;
                let g = pair(2)?;
                let b = pair(4)?;
                Some(RGBColor { r, g, b })
            }
            8 => {
                let pair = |i: usize| -> Option<u8> {
                    u8::from_str_radix(&hex[i..i + 2], 16).ok()
                };
                let r = pair(0)?;
                let g = pair(2)?;
                let b = pair(4)?;
                Some(RGBColor { r, g, b })
            }
            _ => None,
        };
    }
    if let Some(rest) = s.strip_prefix("rgb:") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() != 3 {
            return None;
        }
        let comp = |p: &str| shift(u64::from_str_radix(p, 16).unwrap_or(0));
        return Some(RGBColor {
            r: comp(parts[0]),
            g: comp(parts[1]),
            b: comp(parts[2]),
        });
    }
    if let Some(rest) = s.strip_prefix("rgba:") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() != 4 {
            return None;
        }
        let comp = |p: &str| shift(u64::from_str_radix(p, 16).unwrap_or(0));
        return Some(RGBColor {
            r: comp(parts[0]),
            g: comp(parts[1]),
            b: comp(parts[2]),
        });
    }
    None
}

/// Strips ANSI escape sequences from the given string.
pub fn strip(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(pos) = rest.find('\x1b') {
        out.push_str(&rest[..pos]);
        rest = &rest[pos..];
        let chars: Vec<char> = rest.chars().collect();
        let mut i = 1usize;
        let mut consumed = 1usize;
        if i < chars.len() {
            match chars[i] {
                '[' => {
                    // CSI: consume until the final byte (0x40..=0x7e).
                    i += 1;
                    while i < chars.len() {
                        let c = chars[i];
                        i += 1;
                        consumed = i;
                        if ('\x40'..='\x7e').contains(&c) {
                            break;
                        }
                    }
                }
                ']' => {
                    // OSC: consume until BEL (0x07) or ST (ESC \).
                    i += 1;
                    while i < chars.len() {
                        let c = chars[i];
                        i += 1;
                        consumed = i;
                        if c == '\x07' {
                            break;
                        }
                        if c == '\x1b' && i < chars.len() && chars[i] == '\\' {
                            i += 1;
                            consumed = i;
                            break;
                        }
                    }
                }
                _ => {
                    // Two-character escape (e.g. ESC =, ESC >).
                    consumed = 2;
                }
            }
        }
        rest = &rest[chars[..consumed.min(chars.len())].iter().collect::<String>().len()..];
    }
    out.push_str(rest);
    out
}

/// Cuts the given string from `start` to `end` cell positions, preserving ANSI
/// sequences.
pub fn cut(s: &str, start: usize, end: usize) -> String {
    let mut out = String::new();
    let mut width = 0usize;
    let mut rest = s;
    while !rest.is_empty() {
        if rest.starts_with('\x1b') {
            let chars: Vec<char> = rest.chars().collect();
            let mut i = 1usize;
            let mut consumed = 1usize;
            if i < chars.len() {
                match chars[i] {
                    '[' => {
                        i += 1;
                        while i < chars.len() {
                            let c = chars[i];
                            i += 1;
                            consumed = i;
                            if ('\x40'..='\x7e').contains(&c) {
                                break;
                            }
                        }
                    }
                    ']' => {
                        i += 1;
                        while i < chars.len() {
                            let c = chars[i];
                            i += 1;
                            consumed = i;
                            if c == '\x07' {
                                break;
                            }
                            if c == '\x1b' && i < chars.len() && chars[i] == '\\' {
                                i += 1;
                                consumed = i;
                                break;
                            }
                        }
                    }
                    _ => {
                        consumed = 2;
                    }
                }
            }
            out.push_str(&chars[..consumed.min(chars.len())].iter().collect::<String>());
            rest = &rest[chars[..consumed.min(chars.len())].iter().collect::<String>().len()..];
            continue;
        }
        let c = rest.chars().next().unwrap();
        let w = UnicodeWidthChar::width(c).unwrap_or(0);
        // Upstream (ansi.Cut, GraphemeWidth): a cluster is written when the
        // cumulative width is strictly greater than the left boundary and
        // no greater than the right boundary. This mirrors the combined
        // `truncate` (width <= right) + `truncateLeft` (width > left) pass.
        width += w;
        if width > start && width <= end {
            out.push(c);
        }
        if width > end {
            break;
        }
        rest = &rest[c.len_utf8()..];
    }
    out
}

/// Cuts the given string, keeping only the `max` rightmost cells.
pub fn cut_left(s: &str, max: usize) -> String {
    let width = string_width(s);
    let w = width.saturating_sub(max);
    cut(s, w, width)
}

/// Truncates the given string to `max_width` cells, appending `tail` if
/// truncation occurred. ANSI sequences are preserved.
pub fn truncate(s: &str, max_width: usize, tail: &str) -> String {
    let width = string_width(s);
    if width <= max_width {
        return s.to_string();
    }
    if max_width == 0 {
        return tail.to_string();
    }
    let tw = string_width(tail);
    if tw >= max_width {
        return truncate(tail, max_width, "");
    }
    let cut_at = max_width - tw;
    format!("{}{}", cut(s, 0, cut_at), tail)
}

/// Truncates the given string keeping the rightmost `max_width` cells,
/// prefixed with `head` if truncation occurred.
pub fn truncate_left(s: &str, max_width: usize, head: &str) -> String {
    let width = string_width(s);
    if width <= max_width {
        return s.to_string();
    }
    if max_width == 0 {
        return head.to_string();
    }
    let hw = string_width(head);
    if hw >= max_width {
        return truncate_left(head, max_width, "");
    }
    let keep = max_width - hw;
    format!("{}{}", head, cut(s, width - keep, width))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_width() {
        assert_eq!(string_width("hello"), 5);
        assert_eq!(string_width("你好"), 4);
        assert_eq!(string_width("\x1b[31mhello\x1b[m"), 5);
        assert_eq!(string_width("أهلا"), 4);
    }

    #[test]
    fn test_strip() {
        assert_eq!(strip("\x1b[31mred\x1b[m"), "red");
        assert_eq!(strip("a\x1b]8;;http://x\x07b"), "ab");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello world", 5, "…"), "hell…");
        assert_eq!(truncate("hello world", 8, "…"), "hello w…");
    }

    #[test]
    fn test_x_parse_color() {
        assert_eq!(
            x_parse_color("#ff0000"),
            Some(RGBColor { r: 0xff, g: 0x00, b: 0x00 })
        );
        assert_eq!(
            x_parse_color("#f00"),
            Some(RGBColor { r: 0xff, g: 0x00, b: 0x00 })
        );
        assert_eq!(
            x_parse_color("#0000ff"),
            Some(RGBColor { r: 0x00, g: 0x00, b: 0xff })
        );
        assert_eq!(
            x_parse_color("rgb:ffff/8080/0000"),
            Some(RGBColor { r: 0xff, g: 0x80, b: 0x00 })
        );
        assert_eq!(
            x_parse_color("rgba:ffff/0000/0000/ffff"),
            Some(RGBColor { r: 0xff, g: 0x00, b: 0x00 })
        );
        assert_eq!(x_parse_color("notacolor"), None);
        assert_eq!(x_parse_color("#12345"), None);
        assert_eq!(
            x_parse_color("rgb:ff/00/00"),
            Some(RGBColor { r: 0xff, g: 0x00, b: 0x00 })
        );
        assert_eq!(x_parse_color("rgb:ff/00"), None);
    }
}
