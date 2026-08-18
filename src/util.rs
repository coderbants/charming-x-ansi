//! Cleanroom Rust port of upstream Go source file: `ansi/util.go` and `ansi/width.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! ANSI-aware string utilities: stripping, cutting, truncation, and cell-width
//! measurement.
//! </public-docs>

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
                let nib = |i: usize| -> Option<u8> { u8::from_str_radix(&hex[i..i + 1], 16).ok() };
                let r = nib(0)?.checked_mul(17)?;
                let g = nib(1)?.checked_mul(17)?;
                let b = nib(2)?.checked_mul(17)?;
                Some(RGBColor { r, g, b })
            }
            4 => {
                let nib = |i: usize| -> Option<u8> { u8::from_str_radix(&hex[i..i + 1], 16).ok() };
                let r = nib(0)?.checked_mul(17)?;
                let g = nib(1)?.checked_mul(17)?;
                let b = nib(2)?.checked_mul(17)?;
                Some(RGBColor { r, g, b })
            }
            6 => {
                let pair = |i: usize| -> Option<u8> { u8::from_str_radix(&hex[i..i + 2], 16).ok() };
                let r = pair(0)?;
                let g = pair(2)?;
                let b = pair(4)?;
                Some(RGBColor { r, g, b })
            }
            8 => {
                let pair = |i: usize| -> Option<u8> { u8::from_str_radix(&hex[i..i + 2], 16).ok() };
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
    while let Some(pos) = rest.find('\x1b').or_else(|| {
        rest.char_indices()
            .find(|(_, c)| {
                matches!(
                    c,
                    '\u{9b}' | '\u{9d}' | '\u{90}' | '\u{98}' | '\u{9e}' | '\u{9f}'
                )
            })
            .map(|(i, _)| i)
    }) {
        out.push_str(&rest[..pos]);
        rest = &rest[pos..];
        let (_, len) = take_ansi(rest);
        rest = &rest[len.min(rest.len())..];
    }
    out.push_str(rest);
    out
}

/// Cuts the given string from `start` to `end` cell positions, preserving ANSI
/// sequences. Mirrors upstream `ansi.Cut`: a compose of `truncate` and
/// `truncateLeft`, so trailing ANSI sequences are preserved even when the
/// printable range has ended.
pub fn cut(s: &str, start: usize, end: usize) -> String {
    if end <= start {
        return String::new();
    }
    if start == 0 {
        return truncate(s, end, "");
    }
    truncate_left(&truncate(s, end, ""), start, "")
}

/// Cuts the given string, keeping only the `max` rightmost cells.
pub fn cut_left(s: &str, max: usize) -> String {
    let width = string_width(s);
    let w = width.saturating_sub(max);
    cut(s, w, width)
}

/// Truncates the given string to `max_width` cells, appending `tail` if
/// truncation occurred. ANSI sequences are preserved even after the printable
/// region has been exhausted.
pub fn truncate(s: &str, max_width: usize, tail: &str) -> String {
    if string_width(s) <= max_width {
        return s.to_string();
    }

    let tw = string_width(tail);
    if tw > max_width {
        return String::new();
    }
    let length = max_width - tw;

    let mut buf = String::new();
    let mut cur_width = 0usize;
    let mut ignoring = false;

    let mut rest = s;
    while !rest.is_empty() {
        if starts_ansi(rest) {
            // ANSI sequences are always preserved, even while ignoring.
            let (seq, len) = take_ansi(rest);
            buf.push_str(&seq);
            rest = &rest[len..];
            continue;
        }
        let g = unicode_segmentation::UnicodeSegmentation::graphemes(rest, true)
            .next()
            .unwrap();
        rest = &rest[g.len()..];
        let w = string_width(g);

        if g.len() == 1 {
            let b = g.as_bytes()[0];
            if (0x20..=0x7e).contains(&b) {
                // PrintAction: check the width boundary before incrementing.
                if cur_width >= length && !ignoring {
                    ignoring = true;
                    buf.push_str(tail);
                }
                if ignoring {
                    continue;
                }
                cur_width += 1;
                buf.push_str(g);
                if cur_width > length && !ignoring {
                    ignoring = true;
                    buf.push_str(tail);
                }
                continue;
            }
            // ExecuteAction (controls like \n): written when not ignoring.
            if !ignoring {
                buf.push_str(g);
            }
            if cur_width > length && !ignoring {
                ignoring = true;
                buf.push_str(tail);
            }
            continue;
        }

        // Utf8State: grapheme cluster.
        cur_width += w;
        if ignoring {
            continue;
        }
        if cur_width > length && !ignoring {
            ignoring = true;
            buf.push_str(tail);
        }
        if cur_width > length {
            continue;
        }
        buf.push_str(g);
    }

    buf
}

/// Truncates the given string keeping the rightmost `max_width` cells,
/// prefixed with `head` if truncation occurred.
pub fn truncate_left(s: &str, max_width: usize, head: &str) -> String {
    if max_width == 0 {
        return s.to_string();
    }

    let mut buf = String::new();
    let mut cur_width = 0usize;
    let mut ignoring = true;

    let mut rest = s;
    while !rest.is_empty() {
        if !ignoring {
            buf.push_str(rest);
            break;
        }
        if starts_ansi(rest) {
            let (seq, len) = take_ansi(rest);
            if ignoring {
                // ANSI sequences are preserved even while ignoring.
                buf.push_str(&seq);
            }
            rest = &rest[len..];
            continue;
        }
        let g = unicode_segmentation::UnicodeSegmentation::graphemes(rest, true)
            .next()
            .unwrap();
        rest = &rest[g.len()..];
        let w = string_width(g);

        if g.len() == 1 {
            let b = g.as_bytes()[0];
            if (0x20..=0x7e).contains(&b) {
                // PrintAction.
                cur_width += 1;
                if cur_width > max_width && ignoring {
                    ignoring = false;
                    buf.push_str(head);
                }
                if ignoring {
                    continue;
                }
                buf.push_str(g);
                continue;
            }
            // ExecuteAction (controls like \n): written when not ignoring.
            if !ignoring {
                buf.push_str(g);
            }
            continue;
        }

        // Utf8State: grapheme cluster.
        cur_width += w;
        if cur_width > max_width && ignoring {
            ignoring = false;
            buf.push_str(head);
        }
        if cur_width > max_width {
            buf.push_str(g);
        }
        if ignoring {
            continue;
        }
    }

    buf
}

/// Whether the string starts with an ANSI escape sequence (ESC or C1).
fn starts_ansi(s: &str) -> bool {
    let first = s.chars().next().unwrap_or('\0');
    first == '\x1b'
        || matches!(
            first,
            '\u{9b}' | '\u{9d}' | '\u{90}' | '\u{98}' | '\u{9e}' | '\u{9f}'
        )
}

/// Consumes a full ANSI escape sequence at the start of the string, returning
/// the sequence and its byte length. Handles both ESC-prefixed and 8-bit C1
/// control sequences.
fn take_ansi(s: &str) -> (String, usize) {
    let chars: Vec<char> = s.chars().collect();
    let first = chars[0];
    let is_c1 = matches!(
        first,
        '\u{9b}' | '\u{9d}' | '\u{90}' | '\u{98}' | '\u{9e}' | '\u{9f}'
    );
    let mut i = if is_c1 { 0 } else { 1 };
    let mut consumed = 1usize;

    // Determine the sequence type.
    let kind = if is_c1 {
        if first == '\u{9b}' {
            0 // CSI
        } else {
            1 // string (OSC/DCS/SOS/PM/APC)
        }
    } else if i < chars.len() {
        match chars[i] {
            '[' => 0,                         // CSI
            ']' | 'X' | '^' | '_' | 'P' => 1, // string
            _ => 2,                           // two-char escape
        }
    } else {
        2
    };

    match kind {
        0 => {
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
        1 => {
            // OSC/SOS/PM/APC/DCS: consume until BEL (0x07) or ST (ESC \\ or
            // C1 0x9c).
            i += 1;
            while i < chars.len() {
                let c = chars[i];
                i += 1;
                consumed = i;
                if c == '\x07' || c == '\u{9c}' {
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
            if !is_c1 && i < chars.len() {
                consumed = 2;
            }
        }
    }

    let seq: String = chars[..consumed.min(chars.len())].iter().collect();
    let len = seq.len();
    (seq, len)
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
        // 8-bit C1 forms are stripped too.
        assert_eq!(strip("\u{9b}31mred\u{9b}m"), "red");
        assert_eq!(strip("\u{9d}2;title\u{9c}text"), "text");
        // Multi-byte sequences and bare escapes are removed.
        assert_eq!(strip("\x1b[38;2;1;2;3mhi"), "hi");
        assert_eq!(strip("\x1bPq#0\x1b\\x"), "x");
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
            Some(RGBColor {
                r: 0xff,
                g: 0x00,
                b: 0x00
            })
        );
        assert_eq!(
            x_parse_color("#f00"),
            Some(RGBColor {
                r: 0xff,
                g: 0x00,
                b: 0x00
            })
        );
        assert_eq!(
            x_parse_color("#0000ff"),
            Some(RGBColor {
                r: 0x00,
                g: 0x00,
                b: 0xff
            })
        );
        assert_eq!(
            x_parse_color("rgb:ffff/8080/0000"),
            Some(RGBColor {
                r: 0xff,
                g: 0x80,
                b: 0x00
            })
        );
        assert_eq!(
            x_parse_color("rgba:ffff/0000/0000/ffff"),
            Some(RGBColor {
                r: 0xff,
                g: 0x00,
                b: 0x00
            })
        );
        assert_eq!(x_parse_color("notacolor"), None);
        assert_eq!(x_parse_color("#12345"), None);
        assert_eq!(
            x_parse_color("rgb:ff/00/00"),
            Some(RGBColor {
                r: 0xff,
                g: 0x00,
                b: 0x00
            })
        );
        assert_eq!(x_parse_color("rgb:ff/00"), None);
    }

    /// Ported from upstream `TestTruncate`/`TestTruncateLeft` tcases.
    #[test]
    fn test_truncate_cases() {
        let cases: &[(&str, &str, &str, usize, &str, &str)] = &[
            ("empty", "", "", 0, "", ""),
            ("truncate_length_0", "foo", "", 0, "", "foo"),
            ("equalascii", "one", ".", 3, "one", ""),
            ("equalemoji", "on👋", ".", 3, "on.", ".👋"),
            ("simple multiple words", "a couple of words", "", 6, "a coup", "le of words"),
            ("equalcontrolemoji", "one\x1b[0m", ".", 3, "one\x1b[0m", "\x1b[0m"),
            ("truncate_tail_greater", "foo", "...", 5, "foo", ""),
            ("simple", "foobar", "", 3, "foo", "bar"),
            ("passthrough", "foobar", "", 10, "foobar", ""),
            ("ascii", "hello", "", 3, "hel", "lo"),
            ("emoji", "👋", "", 2, "👋", ""),
            ("wideemoji", "🫧", "", 2, "🫧", ""),
            ("controlemoji", "\x1b[31mhello 👋abc\x1b[0m", "", 8, "\x1b[31mhello 👋\x1b[0m", "\x1b[31mabc\x1b[0m"),
            (
                "osc8",
                "\x1b]8;;https://charm.sh\x1b\\Charmbracelet 🫧\x1b]8;;\x1b\\",
                "",
                5,
                "\x1b]8;;https://charm.sh\x1b\\Charm\x1b]8;;\x1b\\",
                "\x1b]8;;https://charm.sh\x1b\\bracelet 🫧\x1b]8;;\x1b\\",
            ),
            (
                "osc8_8bit",
                "\u{9d}8;;https://charm.sh\u{9c}Charmbracelet 🫧\u{9d}8;;\u{9c}",
                "",
                5,
                "\u{9d}8;;https://charm.sh\u{9c}Charm\u{9d}8;;\u{9c}",
                "\u{9d}8;;https://charm.sh\u{9c}bracelet 🫧\u{9d}8;;\u{9c}",
            ),
            ("style_tail", "\x1b[38;5;219mHiya!", "…", 3, "\x1b[38;5;219mHi…", "\x1b[38;5;219m…a!"),
            (
                "double_style_tail",
                "\x1b[38;5;219mHiya!\x1b[38;5;219mHello",
                "…",
                7,
                "\x1b[38;5;219mHiya!\x1b[38;5;219mH…",
                "\x1b[38;5;219m\x1b[38;5;219m…llo",
            ),
            ("noop", "\x1b[7m--", "", 2, "\x1b[7m--", "\x1b[7m"),
            ("double_width", "\x1b[38;2;249;38;114m你好\x1b[0m", "", 3, "\x1b[38;2;249;38;114m你\x1b[0m", "\x1b[38;2;249;38;114m好\x1b[0m"),
            ("double_width_rune", "你", "", 1, "", "你"),
            ("double_width_runes", "你好", "", 2, "你", "好"),
            ("spaces_only", "    ", "…", 2, " …", "…  "),
            ("longer_tail", "foo", "...", 2, "", "...o"),
            ("same_tail_width", "foo", "...", 3, "foo", ""),
            ("same_tail_width_control", "\x1b[31mfoo\x1b[0m", "...", 3, "\x1b[31mfoo\x1b[0m", "\x1b[31m\x1b[0m"),
            ("same_width", "foo", "", 3, "foo", ""),
            ("truncate_with_tail", "foobar", ".", 4, "foo.", ".ar"),
            (
                "style",
                "I really \x1b[38;2;249;38;114mlove\x1b[0m Go!",
                "",
                8,
                "I really\x1b[38;2;249;38;114m\x1b[0m",
                " \x1b[38;2;249;38;114mlove\x1b[0m Go!",
            ),
            (
                "dcs",
                "\x1bPq#0;2;0;0;0#1;2;100;100;0#2;2;0;100;0#1~~@@vv@@~~@@~~$#2??}}GG}}??}}??-#1!14@\x1b\\foobar",
                "…",
                4,
                "\x1bPq#0;2;0;0;0#1;2;100;100;0#2;2;0;100;0#1~~@@vv@@~~@@~~$#2??}}GG}}??}}??-#1!14@\x1b\\foo…",
                "\x1bPq#0;2;0;0;0#1;2;100;100;0#2;2;0;100;0#1~~@@vv@@~~@@~~$#2??}}GG}}??}}??-#1!14@\x1b\\…ar",
            ),
            ("emoji_tail", "\x1b[36mHello there!\x1b[m", "😃", 8, "\x1b[36mHello 😃\x1b[m", "\x1b[36m😃ere!\x1b[m"),
            ("unicode", "\x1b[35mClaire‘s Boutique\x1b[0m", "", 8, "\x1b[35mClaire‘s\x1b[0m", "\x1b[35m Boutique\x1b[0m"),
            ("wide_chars", "こんにちは", "…", 7, "こんに…", "…ちは"),
            ("style_wide_chars", "\x1b[35mこんにちは\x1b[m", "…", 7, "\x1b[35mこんに…\x1b[m", "\x1b[35m…ちは\x1b[m"),
            (
                "osc8_lf",
                "สวัสดีสวัสดี\x1b]8;;https://example.com\x1b\\\nสวัสดีสวัสดี\x1b]8;;\x1b\\",
                "…",
                9,
                "สวัสดีสวัสดี\x1b]8;;https://example.com\x1b\\\n…\x1b]8;;\x1b\\",
                "\x1b]8;;https://example.com\x1b\\…วัสดีสวัสดี\x1b]8;;\x1b\\",
            ),
            ("simple japanese text prefix/suffix", "耐許ヱヨカハ調出あゆ監", "…", 13, "耐許ヱヨカハ…", "…調出あゆ監"),
            ("simple japanese text", "耐許ヱヨカハ調出あゆ監", "", 14, "耐許ヱヨカハ調", "出あゆ監"),
            (
                "new line inside and outside range",
                "\n\nsomething\nin\nthe\nway\n\n",
                "-",
                10,
                "\n\nsomething\n-",
                "-n\nthe\nway\n\n",
            ),
        ];
        for (name, input, extra, width, right, left) in cases {
            assert_eq!(
                truncate(input, *width, extra),
                *right,
                "Truncate {name}: {input:?}"
            );
            assert_eq!(
                truncate_left(input, *width, extra),
                *left,
                "TruncateLeft {name}: {input:?}"
            );
        }
    }

    /// Ported from upstream `TestCut`.
    #[test]
    fn test_cut_cases() {
        let cases: &[(&str, &str, usize, usize, &str)] = &[
            ("simple string", "This is a long string", 2, 6, "is i"),
            (
                "with ansi",
                "I really \x1b[38;2;249;38;114mlove\x1b[0m Go!",
                4,
                25,
                "ally \x1b[38;2;249;38;114mlove\x1b[0m Go!",
            ),
            (
                "left is 0",
                "Foo \x1b[38;2;249;38;114mbar\x1b[0mbaz",
                0,
                5,
                "Foo \x1b[38;2;249;38;114mb\x1b[0m",
            ),
            ("right is 0", "\x1b[7mHello\x1b[m", 3, 0, ""),
            ("right is less than left", "\x1b[7mHello\x1b[m", 3, 2, ""),
            ("cut size is 0", "\x1b[7mHello\x1b[m", 2, 2, ""),
            (
                "maintains open ansi",
                "\x1b[38;5;212;48;5;63mHello, Artichoke!\x1b[m",
                7,
                16,
                "\x1b[38;5;212;48;5;63mArtichoke\x1b[m",
            ),
            (
                "multiline",
                "\n\x1b[38;2;98;98;98m\nif [ -f RE\nADME.md ]; then\x1b[m\n\x1b[38;2;98;98;98m    echo oi\x1b[m\n\x1b[38;2;98;98;98mfi\x1b[m\n",
                8,
                13,
                "\x1b[38;2;98;98;98mRE\nADM\x1b[m\x1b[38;2;98;98;98m\x1b[m\x1b[38;2;98;98;98m\x1b[m",
            ),
        ];
        for (name, input, left, right, expect) in cases {
            assert_eq!(cut(input, *left, *right), *expect, "Cut {name}: {input:?}");
        }
        // cut_left exercises the underlying cut (Go keeps the boundary cell).
        assert_eq!(cut_left("hello world", 6), " world");
    }
}
