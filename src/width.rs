//! Cleanroom Rust port of upstream Go source file: `ansi/width.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! ANSI-aware cell width measurement for grapheme and wide-character modes.
//! </public-docs>

use crate::method::{east_asian_width, WidthMethod};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

/// StringWidth returns the width of a string in cells. This is the number of
/// cells that the string will occupy when printed in a terminal. ANSI escape
/// codes are ignored and wide characters (such as East Asians and emojis) are
/// accounted for.
/// This treats the text as a sequence of grapheme clusters.
pub fn string_width(s: &str) -> usize {
    string_width_with(s, WidthMethod::GraphemeWidth)
}

/// StringWidthWc returns the width of a string in cells. This is the number of
/// cells that the string will occupy when printed in a terminal. ANSI escape
/// codes are ignored and wide characters (such as East Asians and emojis) are
/// accounted for.
/// This treats the text as a sequence of wide characters and runes.
pub fn string_width_wc(s: &str) -> usize {
    string_width_with(s, WidthMethod::WcWidth)
}

fn string_width_with(s: &str, m: WidthMethod) -> usize {
    if s.is_empty() {
        return 0;
    }

    let mut width = 0usize;
    let mut state = State::Ground;
    let mut i = 0usize;
    while i < s.len() {
        let c = s[i..].chars().next().unwrap();
        let cp = c as u32;
        match state {
            State::Ground => match c {
                '\x1b' => {
                    state = State::Esc;
                    i += c.len_utf8();
                }
                '\u{0090}' => {
                    state = State::Dcs;
                    i += c.len_utf8();
                }
                '\u{0098}' | '\u{009e}' | '\u{009f}' => {
                    state = State::Str;
                    i += c.len_utf8();
                }
                '\u{009b}' => {
                    state = State::Csi;
                    i += c.len_utf8();
                }
                '\u{009d}' => {
                    state = State::Osc;
                    i += c.len_utf8();
                }
                c if cp <= 0x1f || cp == 0x7f || (0x80..=0x9f).contains(&cp) => {
                    i += c.len_utf8();
                }
                _ => {
                    let cluster = first_grapheme_cluster(&s[i..]).unwrap();
                    width += cluster_width(cluster, m);
                    i += cluster.len();
                }
            },
            State::Esc => match c {
                '\x1b' => {
                    i += c.len_utf8();
                }
                'P' => {
                    state = State::Dcs;
                    i += c.len_utf8();
                }
                '[' => {
                    state = State::Csi;
                    i += c.len_utf8();
                }
                ']' => {
                    state = State::Osc;
                    i += c.len_utf8();
                }
                'X' | '^' | '_' => {
                    state = State::Str;
                    i += c.len_utf8();
                }
                '\u{0098}' | '\u{009e}' | '\u{009f}' | '\u{0090}' | '\u{009b}' | '\u{009d}' => {
                    state = State::Ground;
                }
                c if (0x20..=0x2f).contains(&(c as u32)) => {
                    state = State::EscInter;
                    i += c.len_utf8();
                }
                c if (0x30..=0x4f).contains(&(c as u32))
                    || (0x51..=0x57).contains(&(c as u32))
                    || (0x59..=0x5a).contains(&(c as u32))
                    || c == '\u{005c}'
                    || (0x60..=0x7e).contains(&(c as u32)) =>
                {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                c if (0x80..=0x9f).contains(&(c as u32)) => {
                    state = State::Ground;
                }
                c if cp <= 0x1f || cp == 0x7f => {
                    i += c.len_utf8();
                }
                _ => {
                    state = State::Ground;
                }
            },
            State::EscInter => match c {
                '\x1b' => {
                    state = State::Esc;
                    i += c.len_utf8();
                }
                '\u{0098}' | '\u{009e}' | '\u{009f}' | '\u{0090}' | '\u{009b}' | '\u{009d}' => {
                    state = State::Ground;
                }
                c if (0x30..=0x7e).contains(&(c as u32)) => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                c if (0x80..=0x9f).contains(&(c as u32)) => {
                    state = State::Ground;
                }
                c if cp <= 0x1f || cp == 0x7f => {
                    i += c.len_utf8();
                }
                _ => {
                    i += c.len_utf8();
                }
            },
            State::Csi => match c {
                '\x1b' => {
                    state = State::Esc;
                    i += c.len_utf8();
                }
                '\u{009b}' | '\u{0090}' | '\u{009d}' | '\u{0098}' | '\u{009e}' | '\u{009f}' => {
                    state = State::Ground;
                }
                c if (0x40..=0x7e).contains(&(c as u32)) => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                c if cp == 0x18 || cp == 0x1a || cp == 0x9c => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                c if (0x80..=0x9f).contains(&(c as u32)) => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                c if cp <= 0x1f || cp == 0x7f => {
                    i += c.len_utf8();
                }
                _ => {
                    i += c.len_utf8();
                }
            },
            State::Osc => match c {
                '\x1b' => {
                    state = State::Esc;
                    i += c.len_utf8();
                }
                '\u{0007}' => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                '\u{009c}' => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                '\u{0098}' | '\u{009e}' | '\u{009f}' | '\u{0090}' | '\u{009b}' | '\u{009d}' => {
                    state = State::Ground;
                }
                c if cp == 0x18 || cp == 0x1a => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                c if (0x80..=0x9f).contains(&(c as u32)) => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                _ => {
                    i += c.len_utf8();
                }
            },
            State::Str => match c {
                '\x1b' => {
                    state = State::Esc;
                    i += c.len_utf8();
                }
                '\u{009c}' => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                '\u{0098}' | '\u{009e}' | '\u{009f}' | '\u{0090}' | '\u{009b}' | '\u{009d}' => {
                    state = State::Ground;
                }
                c if cp == 0x18 || cp == 0x1a => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                c if (0x80..=0x9f).contains(&(c as u32)) => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                _ => {
                    i += c.len_utf8();
                }
            },
            State::Dcs => match c {
                '\x1b' => {
                    state = State::Esc;
                    i += c.len_utf8();
                }
                '\u{009c}' => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                '\u{0098}' | '\u{009e}' | '\u{009f}' | '\u{0090}' | '\u{009b}' | '\u{009d}' => {
                    state = State::Ground;
                }
                c if cp == 0x18 || cp == 0x1a => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                c if (0x80..=0x9f).contains(&(c as u32)) => {
                    state = State::Ground;
                    i += c.len_utf8();
                }
                _ => {
                    i += c.len_utf8();
                }
            },
        }
    }
    width
}

/// Returns the width of the first grapheme cluster of the string.
pub fn first_grapheme_cluster(s: &str) -> Option<&str> {
    UnicodeSegmentation::graphemes(s, true).next()
}

/// The subset of the ANSI parser states needed for width measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum State {
    #[default]
    Ground,
    Esc,
    EscInter,
    Csi,
    Dcs,
    Osc,
    Str,
}

fn cluster_width(cluster: &str, m: WidthMethod) -> usize {
    match m {
        WidthMethod::WcWidth => wc_width(cluster),
        WidthMethod::GraphemeWidth => grapheme_width(cluster),
    }
}

/// The width of a single grapheme cluster, measured as the width of its first
/// non-zero-width rune, mirroring `runewidth.Condition.StringWidth`.
fn wc_width(cluster: &str) -> usize {
    for c in cluster.chars() {
        let w = if east_asian_width() {
            UnicodeWidthChar::width_cjk(c)
        } else {
            UnicodeWidthChar::width(c)
        }
        .unwrap_or(0);
        if w > 0 {
            return w;
        }
    }
    0
}

/// The width of a single grapheme cluster per `displaywidth.Options.String`,
/// which follows the Unicode East Asian Width property of the cluster.
fn grapheme_width(cluster: &str) -> usize {
    let bytes = cluster.as_bytes();
    if bytes.len() == 1 {
        return if bytes[0] <= 0x1f || bytes[0] == 0x7f {
            0
        } else {
            1
        };
    }
    let first = cluster.chars().next().unwrap();
    let cp = first as u32;
    if cp <= 0x1f {
        return 0;
    }
    if (0x1f1e6..=0x1f1ff).contains(&cp) {
        return 2;
    }
    if cluster[first.len_utf8()..].starts_with('\u{fe0f}') {
        return 2;
    }
    if east_asian_width() {
        UnicodeWidthChar::width_cjk(first)
    } else {
        UnicodeWidthChar::width(first)
    }
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&str, &str, &str, usize, usize)] = &[
        ("empty", "", "", 0, 0),
        ("ascii", "hello", "hello", 5, 5),
        ("emoji", "👋", "👋", 2, 2),
        ("wideemoji", "🫧", "🫧", 2, 2),
        ("combining", "a\u{0300}", "a\u{0300}", 1, 1),
        ("control", "\x1b[31mhello\x1b[0m", "hello", 5, 5),
        ("csi8", "\u{9b}38;5;1mhello\u{9b}m", "hello", 5, 5),
        (
            "osc",
            "\u{9d}2;charmbracelet: ~/Source/bubbletea\u{9c}",
            "",
            0,
            0,
        ),
        ("controlemoji", "\x1b[31m👋\x1b[0m", "👋", 2, 2),
        (
            "oscwideemoji",
            "\x1b]2;title👨\u{200d}👩\u{200d}👦\x07",
            "",
            0,
            0,
        ),
        (
            "oscwideemoji",
            "\x1b[31m👨\u{200d}👩\u{200d}👦\x1b[m",
            "👨\u{200d}👩\u{200d}👦",
            2,
            2,
        ),
        (
            "multiemojicsi",
            "👨\u{200d}👩\u{200d}👦\u{9b}38;5;1mhello\u{9b}m",
            "👨\u{200d}👩\u{200d}👦hello",
            7,
            7,
        ),
        (
            "osc8eastasianlink",
            "\u{9d}8;id=1;https://example.com/\u{9c}打豆豆\u{9d}8;id=1;\x07",
            "打豆豆",
            6,
            6,
        ),
        ("dcsarabic", "\x1bP?123$pسلام\x1b\\اهلا", "اهلا", 4, 4),
        ("newline", "hello\nworld", "hello\nworld", 10, 10),
        ("tab", "hello\tworld", "hello\tworld", 10, 10),
        (
            "controlnewline",
            "\x1b[31mhello\x1b[0m\nworld",
            "hello\nworld",
            10,
            10,
        ),
        ("style", "\x1B[38;2;249;38;114mfoo", "foo", 3, 3),
        ("unicode", "\x1b[35m“box”\x1b[0m", "“box”", 5, 5),
        (
            "just_unicode",
            "Claire’s Boutique",
            "Claire’s Boutique",
            17,
            17,
        ),
        ("unclosed_ansi", "Hey, \x1b[7m\n猴", "Hey, \n猴", 7, 7),
        ("double_asian_runes", " 你\x1b[8m好.", " 你好.", 6, 6),
        ("flag", "🇸🇦", "🇸🇦", 2, 1),
        ("half width and ascii", "(ﾟ", "(ﾟ", 1, 1),
    ];

    #[test]
    fn test_string_width() {
        for (name, input, _, width, _) in CASES {
            let got = string_width(input);
            assert_eq!(
                got, *width,
                "case {name}: StringWidth({input:?}) = {got}, want {width}"
            );
        }
    }

    #[test]
    fn test_string_width_wc() {
        for (name, input, _, _, wcwidth) in CASES {
            let got = string_width_wc(input);
            assert_eq!(
                got, *wcwidth,
                "case {name}: StringWidthWc({input:?}) = {got}, want {wcwidth}"
            );
        }
    }

    #[test]
    fn test_first_grapheme_cluster() {
        assert_eq!(first_grapheme_cluster("hello"), Some("h"));
        assert_eq!(first_grapheme_cluster("a\u{0300}b"), Some("a\u{0300}"));
        assert_eq!(first_grapheme_cluster(""), None);
    }
}
