//! Cleanroom Rust port of upstream Go source file: `ansi/wrap.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! Word wrapping that preserves ANSI escape sequences and wide characters.
//! </public-docs>

use crate::util::string_width;

const NBSP: char = '\u{00A0}';

/// Wraps a string or a block of text to a given line length, breaking word
/// boundaries if necessary. This preserves ANSI escape codes and accounts for
/// wide characters. The breakpoints string is a list of characters that are
/// considered breakpoints for word wrapping. A hyphen (`-`) is always a
/// breakpoint.
///
/// This treats the text as a sequence of graphemes.
pub fn wrap(s: &str, limit: usize, breakpoints: &str) -> String {
    wrap_impl(s, limit, breakpoints)
}

/// Wraps with the wide-character width method.
pub fn wrap_wc(s: &str, limit: usize, breakpoints: &str) -> String {
    wrap_impl(s, limit, breakpoints)
}

/// Wordwrap wraps a string or a block of text to a given line length, not
/// breaking word boundaries. This preserves ANSI escape codes and accounts for
/// wide characters. The breakpoints string is a list of characters that are
/// considered breakpoints for word wrapping. A hyphen (`-`) is always a
/// breakpoint.
///
/// This treats the text as a sequence of graphemes.
pub fn wordwrap(s: &str, limit: usize, breakpoints: &str) -> String {
    wordwrap_impl(s, limit, breakpoints)
}

/// Wordwrap with the wide-character width method.
pub fn wordwrap_wc(s: &str, limit: usize, breakpoints: &str) -> String {
    wordwrap_impl(s, limit, breakpoints)
}

/// Wraps a string or a block of text to a given line length, breaking word
/// boundaries. This preserves ANSI escape codes. When `preserve_space` is
/// true, spaces at the beginning of a line will be preserved.
pub fn hardwrap(s: &str, limit: usize, preserve_space: bool) -> String {
    hardwrap_impl(s, limit, preserve_space)
}

/// Hardwrap with the wide-character width method.
pub fn hardwrap_wc(s: &str, limit: usize, preserve_space: bool) -> String {
    hardwrap_impl(s, limit, preserve_space)
}

#[allow(unused_assignments)]
fn hardwrap_impl(s: &str, limit: usize, preserve_space: bool) -> String {
    if limit < 1 {
        return s.to_string();
    }
    let mut buf = String::new();
    let mut cur_width = 0usize;
    let mut force_newline = false;
    let mut rest = s;

    while !rest.is_empty() {
        if rest.starts_with('\x1b') {
            let (seq, len) = take_escape(rest);
            buf.push_str(&seq);
            rest = &rest[len..];
            continue;
        }

        let g = unicode_segmentation::UnicodeSegmentation::graphemes(rest, true)
            .next()
            .unwrap();
        let r = g.chars().next().unwrap();

        if g.len() == 1 && (r as u32) <= 0x7f {
            // Single-byte ASCII: the parser's Print/Execute path uses +1 width.
            if r == '\n' {
                buf.push('\n');
                cur_width = 0;
                force_newline = false;
                rest = &rest[1..];
                continue;
            }
            if cur_width + 1 > limit {
                buf.push('\n');
                cur_width = 0;
                force_newline = true;
            }
            if cur_width == 0 {
                if !preserve_space && force_newline && r.is_whitespace() {
                    rest = &rest[1..];
                    continue;
                }
                force_newline = false;
            }
            buf.push_str(g);
            // PrintAction increments the width; ExecuteAction (C0/DEL) does not.
            if (r as u32) >= 0x20 && r != '\u{7f}' {
                cur_width += 1;
            }
        } else {
            let w = string_width(g);
            if cur_width + w > limit {
                buf.push('\n');
                cur_width = 0;
            }
            if !preserve_space && cur_width == 0 && g.len() <= 4 && r.is_whitespace() {
                rest = &rest[g.len()..];
                continue;
            }
            buf.push_str(g);
            cur_width += w;
        }
        rest = &rest[g.len()..];
    }

    buf
}

#[allow(unused_assignments)]
fn wordwrap_impl(s: &str, limit: usize, breakpoints: &str) -> String {
    if limit < 1 {
        return s.to_string();
    }
    let breakpoints: Vec<char> = breakpoints.chars().collect();

    let mut buf = String::new();
    let mut word = String::new();
    let mut space = String::new();
    let mut space_width = 0usize;
    let mut cur_width = 0usize;
    let mut word_len = 0usize;

    macro_rules! add_space {
        () => {
            if space_width != 0 || !space.is_empty() {
                cur_width += space_width;
                buf.push_str(&space);
                space.clear();
                space_width = 0;
            }
        };
    }
    macro_rules! add_word {
        () => {
            if !word.is_empty() {
                add_space!();
                cur_width += word_len;
                buf.push_str(&word);
                word.clear();
                word_len = 0;
            }
        };
    }
    macro_rules! add_newline {
        () => {
            buf.push('\n');
            cur_width = 0;
            space.clear();
            space_width = 0;
        };
    }

    let mut rest = s;
    while !rest.is_empty() {
        let token;
        let is_esc;
        if rest.starts_with('\x1b') {
            let (seq, len) = take_escape(rest);
            token = seq;
            is_esc = true;
            rest = &rest[len..];
        } else {
            let g = unicode_segmentation::UnicodeSegmentation::graphemes(rest, true)
                .next()
                .unwrap();
            token = g.to_string();
            is_esc = false;
            rest = &rest[g.len()..];
        }

        if is_esc {
            word.push_str(&token);
            continue;
        }

        let r = token.chars().next().unwrap();
        let w = string_width(&token);
        if r == '\n' {
            if word_len == 0 {
                if cur_width + space_width > limit {
                    cur_width = 0;
                } else {
                    buf.push_str(&space);
                }
                space.clear();
                space_width = 0;
            }
            add_word!();
            add_newline!();
        } else if r.is_whitespace() && r != NBSP {
            add_word!();
            space.push_str(&token);
            space_width += w;
        } else if breakpoints.contains(&r) || r == '-' {
            add_space!();
            add_word!();
            buf.push_str(&token);
            cur_width += w;
        } else {
            word.push_str(&token);
            word_len += w;
            if cur_width + space_width + word_len > limit && word_len < limit {
                add_newline!();
            }
        }
    }

    add_word!();

    buf
}

#[allow(unused_assignments)]
fn wrap_impl(s: &str, limit: usize, breakpoints: &str) -> String {
    if limit < 1 {
        return s.to_string();
    }
    let breakpoints: Vec<char> = breakpoints.chars().collect();

    let mut buf = String::new();
    let mut word = String::new();
    let mut space = String::new();
    let mut space_width = 0usize;
    let mut cur_width = 0usize;
    let mut word_len = 0usize;

    macro_rules! add_space {
        () => {
            if space_width == 0 && space.is_empty() {
                // no-op
            } else {
                cur_width += space_width;
                buf.push_str(&space);
                space.clear();
                space_width = 0;
            }
        };
    }
    macro_rules! add_word {
        () => {
            if !word.is_empty() {
                add_space!();
                cur_width += word_len;
                buf.push_str(&word);
                word.clear();
                word_len = 0;
            }
        };
    }
    macro_rules! add_newline {
        () => {
            buf.push('\n');
            cur_width = 0;
            space.clear();
            space_width = 0;
        };
    }

    let mut rest = s;
    while !rest.is_empty() {
        let token;
        let is_esc;
        if rest.starts_with('\x1b') {
            let (seq, len) = take_escape(rest);
            token = seq;
            is_esc = true;
            rest = &rest[len..];
        } else {
            let g = unicode_segmentation::UnicodeSegmentation::graphemes(rest, true)
                .next()
                .unwrap();
            token = g.to_string();
            is_esc = false;
            rest = &rest[g.len()..];
        }

        if is_esc {
            word.push_str(&token);
            continue;
        }

        let r = token.chars().next().unwrap();
        let w = string_width(&token);
        if r == '\n' {
            if word_len == 0 {
                if cur_width + space_width > limit {
                    cur_width = 0;
                } else {
                    buf.push_str(&space);
                }
                space.clear();
                space_width = 0;
            }
            add_word!();
            add_newline!();
        } else if r.is_whitespace() && r != NBSP {
            add_word!();
            space.push_str(&token);
            space_width += w;
        } else if breakpoints.contains(&r) || r == '-' {
            add_space!();
            if cur_width + word_len + w > limit {
                word.push_str(&token);
                word_len += w;
            } else {
                add_word!();
                buf.push_str(&token);
                cur_width += w;
            }
        } else {
            if word_len + w > limit {
                add_word!();
            }
            word.push_str(&token);
            word_len += w;
            if cur_width + word_len + space_width > limit {
                add_newline!();
            }
            if word_len == limit {
                add_word!();
            }
        }
    }

    if word_len == 0 {
        if cur_width + space_width > limit {
            cur_width = 0;
        } else {
            buf.push_str(&space);
        }
        space.clear();
        space_width = 0;
    }
    add_word!();

    buf
}

/// Consumes a full ANSI escape sequence at the start of the string, returning
/// the sequence and its byte length. Handles CSI, OSC, DCS, SOS, PM, APC, and
/// single-character escapes (including ST, ESC \).
fn take_escape(s: &str) -> (String, usize) {
    let chars: Vec<char> = s.chars().collect();
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
            ']' | 'X' | '^' | '_' => {
                // OSC/SOS/PM/APC: consume until BEL (0x07) or ST (ESC \\).
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
            'P' => {
                // DCS: consume until ST (ESC \\).
                i += 1;
                while i < chars.len() {
                    let c = chars[i];
                    i += 1;
                    consumed = i;
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
    let seq: String = chars[..consumed.min(chars.len())].iter().collect();
    let len = seq.len();
    (seq, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&str, &str, usize, &str, bool)] = &[
        ("empty string", "", 0, "", true),
        ("passthrough", "foobar\n ", 0, "foobar\n ", true),
        ("pass", "foo", 4, "foo", true),
        ("simple", "foobarfoo", 4, "foob\narfo\no", true),
        ("lf", "f\no\nobar", 3, "f\no\noba\nr", true),
        ("lf_space", "foo bar\n  baz", 3, "foo\n ba\nr\n  b\naz", true),
        ("tab", "foo\tbar", 3, "foo\n\tbar", true),
        (
            "unicode_space",
            "foo\u{a0}bar",
            3,
            "foo\nbar",
            false,
        ),
        (
            "style_nochange",
            "\x1b[38;2;249;38;114mfoo\x1b[0m\x1b[38;2;248;248;242m \x1b[0m\x1b[38;2;230;219;116mbar\x1b[0m",
            7,
            "\x1b[38;2;249;38;114mfoo\x1b[0m\x1b[38;2;248;248;242m \x1b[0m\x1b[38;2;230;219;116mbar\x1b[0m",
            true,
        ),
        (
            "style",
            "\x1b[38;2;249;38;114m(\x1b[0m\x1b[38;2;248;248;242mjust another test\x1b[38;2;249;38;114m)\x1b[0m",
            3,
            "\x1b[38;2;249;38;114m(\x1b[0m\x1b[38;2;248;248;242mju\nst \nano\nthe\nr t\nest\x1b[38;2;249;38;114m\n)\x1b[0m",
            true,
        ),
        (
            "style_lf",
            "I really \x1b[38;2;249;38;114mlove\x1b[0m Go!",
            8,
            "I really\n\x1b[38;2;249;38;114mlove\x1b[0m Go!",
            false,
        ),
        (
            "style_emoji",
            "I really \x1b[38;2;249;38;114mlove u🫧\x1b[0m",
            8,
            "I really\n\x1b[38;2;249;38;114mlove u🫧\x1b[0m",
            false,
        ),
        (
            "hyperlink",
            "I really \x1b]8;;https://example.com/\x1b\\love\x1b]8;;\x1b\\ Go!",
            10,
            "I really \x1b]8;;https://example.com/\x1b\\l\nove\x1b]8;;\x1b\\ Go!",
            false,
        ),
        (
            "dcs",
            "\x1bPq#0;2;0;0;0#1;2;100;100;0#2;2;0;100;0#1~~@@vv@@~~@@~~$#2??}}GG}}??}}??-#1!14@\x1b\\foobar",
            3,
            "\x1bPq#0;2;0;0;0#1;2;100;100;0#2;2;0;100;0#1~~@@vv@@~~@@~~$#2??}}GG}}??}}??-#1!14@\x1b\\foo\nbar",
            false,
        ),
        ("begin_with_space", " foo", 4, " foo", false),
        (
            "style_dont_affect_wrap",
            "\x1b[38;2;249;38;114mfoo\x1b[0m\x1b[38;2;248;248;242m \x1b[0m\x1b[38;2;230;219;116mbar\x1b[0m",
            7,
            "\x1b[38;2;249;38;114mfoo\x1b[0m\x1b[38;2;248;248;242m \x1b[0m\x1b[38;2;230;219;116mbar\x1b[0m",
            false,
        ),
        (
            "preserve_style",
            "\x1b[38;2;249;38;114m(\x1b[0m\x1b[38;2;248;248;242mjust another test\x1b[38;2;249;38;114m)\x1b[0m",
            3,
            "\x1b[38;2;249;38;114m(\x1b[0m\x1b[38;2;248;248;242mju\nst \nano\nthe\nr t\nest\x1b[38;2;249;38;114m\n)\x1b[0m",
            false,
        ),
        ("emoji", "foo🫧foobar", 4, "foo\n🫧fo\nobar", false),
        (
            "osc8_wrap",
            "สวัสดีสวัสดี\x1b]8;;https://example.com\x1b\\สวัสดีสวัสดี\x1b]8;;\x1b\\",
            8,
            "สวัสดีสวัสดี\x1b]8;;https://example.com\x1b\\\nสวัสดีสวัสดี\x1b]8;;\x1b\\",
            false,
        ),
        ("column", "VERTICAL", 1, "V\nE\nR\nT\nI\nC\nA\nL", false),
    ];

    #[test]
    fn test_hardwrap() {
        for (name, input, limit, expected, preserve_space) in CASES {
            let got = hardwrap(input, *limit, *preserve_space);
            assert_eq!(got, *expected, "hardwrap case {name}: {input:?}");
        }
    }

    #[test]
    fn test_wrap_basic() {
        assert_eq!(
            wrap("The quick brown fox jumps over the lazy dog", 10, ""),
            "The quick\nbrown fox\njumps over\nthe lazy\ndog"
        );
    }

    #[test]
    fn test_wrap_preserves_whitespace() {
        assert_eq!(wrap("  Glossier", 32, ""), "  Glossier");
        assert_eq!(wrap("  Glossier  ", 32, ""), "  Glossier  ");
    }

    #[test]
    fn test_wrap_short() {
        assert_eq!(wrap("aaa bbb ccc", 3, ""), "aaa\nbbb\nccc");
    }

    const WW_CASES: &[(&str, &str, usize, &str, &str)] = &[
        ("empty string", "", 0, "", ""),
        ("passthrough", "foobar\n ", 0, "", "foobar\n "),
        ("pass", "foo", 3, "", "foo"),
        ("toolong", "foobarfoo", 4, "", "foobarfoo"),
        ("white space", "foo bar foo", 4, "", "foo\nbar\nfoo"),
        ("broken_at_spaces", "foo bars foobars", 4, "", "foo\nbars\nfoobars"),
        ("hyphen", "foo-foobar", 4, "-", "foo-\nfoobar"),
        ("emoji_breakpoint", "foo😃 foobar", 4, "😃", "foo😃\nfoobar"),
        ("wide_emoji_breakpoint", "foo🫧 foobar", 4, "🫧", "foo🫧\nfoobar"),
        ("space_breakpoint", "foo --bar", 9, "-", "foo --bar"),
        ("simple", "foo bars foobars", 4, "", "foo\nbars\nfoobars"),
        ("limit", "foo bar", 5, "", "foo\nbar"),
        ("remove white spaces", "foo    \nb   ar   ", 4, "", "foo\nb\nar"),
        ("white space trail width", "foo\nb\t a\n bar", 4, "", "foo\nb\t a\n bar"),
        ("explicit_line_break", "foo bar foo\n", 4, "", "foo\nbar\nfoo\n"),
        ("explicit_breaks", "\nfoo bar\n\n\nfoo\n", 4, "", "\nfoo\nbar\n\n\nfoo\n"),
        (
            "example",
            " This is a list: \n\n\t* foo\n\t* bar\n\n\n\t* foo  \nbar    ",
            6,
            "",
            " This\nis a\nlist: \n\n\t* foo\n\t* bar\n\n\n\t* foo\nbar",
        ),
        (
            "style_code_dont_affect_length",
            "\x1b[38;2;249;38;114mfoo\x1b[0m\x1b[38;2;248;248;242m \x1b[0m\x1b[38;2;230;219;116mbar\x1b[0m",
            7,
            "",
            "\x1b[38;2;249;38;114mfoo\x1b[0m\x1b[38;2;248;248;242m \x1b[0m\x1b[38;2;230;219;116mbar\x1b[0m",
        ),
        (
            "style_code_dont_get_wrapped",
            "\x1b[38;2;249;38;114m(\x1b[0m\x1b[38;2;248;248;242mjust another test\x1b[38;2;249;38;114m)\x1b[0m",
            3,
            "",
            "\x1b[38;2;249;38;114m(\x1b[0m\x1b[38;2;248;248;242mjust\nanother\ntest\x1b[38;2;249;38;114m)\x1b[0m",
        ),
        (
            "osc8_wrap",
            "สวัสดีสวัสดี\x1b]8;;https://example.com\x1b\\ สวัสดีสวัสดี\x1b]8;;\x1b\\",
            8,
            "",
            "สวัสดีสวัสดี\x1b]8;;https://example.com\x1b\\\nสวัสดีสวัสดี\x1b]8;;\x1b\\",
        ),
    ];

    #[test]
    fn test_wordwrap() {
        for (name, input, limit, breakpoints, expected) in WW_CASES {
            let got = wordwrap(input, *limit, breakpoints);
            assert_eq!(got, *expected, "wordwrap case {name}: {input:?}");
        }
        // Wc variant shares the implementation.
        assert_eq!(wordwrap_wc("foo bar foo", 4, ""), "foo\nbar\nfoo");
    }

    #[test]
    fn test_wrap_wordwrap() {
        let input = "the quick brown foxxxxxxxxxxxxxxxx jumped over the lazy dog.";
        let output = wrap(input, 16, "");
        assert_eq!(
            output,
            "the quick brown\nfoxxxxxxxxxxxxxx\nxx jumped over\nthe lazy dog."
        );
    }

    const WRAP_CASES: &[(&str, &str, &str, usize)] = &[
        (
            "simple",
            "I really \x1b[38;2;249;38;114mlove\x1b[0m Go!",
            "I really\n\x1b[38;2;249;38;114mlove\x1b[0m Go!",
            8,
        ),
        ("passthrough", "hello world", "hello world", 11),
        ("asian", "こんにち", "こんに\nち", 7),
        ("emoji", "😃👰🏻‍♀️🫧", "😃\n👰🏻‍♀️\n🫧", 2),
        (
            "long style",
            "\x1b[38;2;249;38;114ma really long string\x1b[0m",
            "\x1b[38;2;249;38;114ma really\nlong\nstring\x1b[0m",
            10,
        ),
        (
            "long style nbsp",
            "\x1b[38;2;249;38;114ma really\u{a0}long string\x1b[0m",
            "\x1b[38;2;249;38;114ma\nreally\u{a0}lon\ng string\x1b[0m",
            10,
        ),
        (
            "longer",
            "the quick brown foxxxxxxxxxxxxxxxx jumped over the lazy dog.",
            "the quick brown\nfoxxxxxxxxxxxxxx\nxx jumped over\nthe lazy dog.",
            16,
        ),
        (
            "longer asian",
            "猴 猴 猴猴 猴猴猴猴猴猴猴猴猴 猴猴猴 猴猴 猴’ 猴猴 猴.",
            "猴 猴 猴猴\n猴猴猴猴猴猴猴猴\n猴 猴猴猴 猴猴\n猴’ 猴猴 猴.",
            16,
        ),
        (
            "long input",
            "Rotated keys for a-good-offensive-cheat-code-incorporated/animal-like-law-on-the-rocks.",
            "Rotated keys for a-good-offensive-cheat-code-incorporated/animal-like-law-\non-the-rocks.",
            76,
        ),
        (
            "long input2",
            "Rotated keys for a-good-offensive-cheat-code-incorporated/crypto-line-operating-system.",
            "Rotated keys for a-good-offensive-cheat-code-incorporated/crypto-line-\noperating-system.",
            76,
        ),
        (
            "hyphen breakpoint",
            "a-good-offensive-cheat-code",
            "a-good-\noffensive-\ncheat-code",
            10,
        ),
        ("exact", "\x1b[91mfoo\x1b[0", "\x1b[91mfoo\x1b[0", 3),
        ("extra space", "foo ", "foo", 3),
        ("extra space style", "\x1b[mfoo \x1b[m", "\x1b[mfoo\x1b[m", 3),
        ("hyphen break", "foo-bar", "foo-\nbar", 5),
        ("double space", "f  bar foobaz", "f  bar\nfoobaz", 6),
        ("passthrough", "foobar\n ", "foobar\n ", 0),
        ("pass", "foo", "foo", 3),
        ("toolong", "foobarfoo", "foob\narfo\no", 4),
        ("white space", "foo bar foo", "foo\nbar\nfoo", 4),
        ("broken_at_spaces", "foo bars foobars", "foo\nbars\nfoob\nars", 4),
        ("hyphen", "foob-foobar", "foob\n-foo\nbar", 4),
        ("wide_emoji_breakpoint", "foo🫧 foobar", "foo\n🫧\nfoob\nar", 4),
        ("space_breakpoint", "foo --bar", "foo --bar", 9),
        ("simple", "foo bars foobars", "foo\nbars\nfoob\nars", 4),
        ("limit", "foo bar", "foo\nbar", 5),
        ("remove white spaces", "foo    \nb   ar   ", "foo\nb\nar", 4),
        ("white space trail width", "foo\nb\t a\n bar", "foo\nb\t a\n bar", 4),
        ("explicit_line_break", "foo bar foo\n", "foo\nbar\nfoo\n", 4),
        ("explicit_breaks", "\nfoo bar\n\n\nfoo\n", "\nfoo\nbar\n\n\nfoo\n", 4),
        (
            "example",
            " This is a list: \n\n\t* foo\n\t* bar\n\n\n\t* foo  \nbar    ",
            " This\nis a\nlist: \n\n\t* foo\n\t* bar\n\n\n\t* foo\nbar",
            6,
        ),
        (
            "style_code_dont_affect_length",
            "\x1b[38;2;249;38;114mfoo\x1b[0m\x1b[38;2;248;248;242m \x1b[0m\x1b[38;2;230;219;116mbar\x1b[0m",
            "\x1b[38;2;249;38;114mfoo\x1b[0m\x1b[38;2;248;248;242m \x1b[0m\x1b[38;2;230;219;116mbar\x1b[0m",
            7,
        ),
        (
            "style_code_dont_get_wrapped",
            "\x1b[38;2;249;38;114m(\x1b[0m\x1b[38;2;248;248;242mjust another test\x1b[38;2;249;38;114m)\x1b[0m",
            "\x1b[38;2;249;38;114m(\x1b[0m\x1b[38;2;248;248;242mjust\nanother\ntest\x1b[38;2;249;38;114m)\x1b[0m",
            7,
        ),
        (
            "osc8_wrap",
            "สวัสดีสวัสดี\x1b]8;;https://example.com\x1b\\ สวัสดีสวัสดี\x1b]8;;\x1b\\",
            "สวัสดีสวัสดี\x1b]8;;https://example.com\x1b\\\nสวัสดีสวัสดี\x1b]8;;\x1b\\",
            8,
        ),
        ("tab", "foo\tbar", "foo\nbar", 3),
        ("Narrow NBSP", "0\u{202f}1\u{202f}2\u{202f}3\u{202f}4", "0\u{202f}1\u{202f}2\u{202f}3\n4", 7),
        (
            "Paragraph Separator",
            "0\u{2029}1\u{2029}2\u{2029}3\u{2029}4",
            "0\u{2029}1\u{2029}2\u{2029}3\u{2029}4",
            7,
        ),
        ("Medium Mathematical Space", "0\u{205f}1\u{205f}2\u{205f}3\u{205f}4", "0\u{205f}1\u{205f}2\u{205f}3\n4", 7),
        ("Ideagraphic space", "0\u{3000}1\u{3000}2\u{3000}3\u{3000}", "0\u{3000}1\u{3000}2\n3\u{3000}", 7),
        (
            "Multi Byte spaces",
            "A\u{202f}B\u{202f}C\u{202f}DA\u{205f}\u{205f}B\u{205f}C\u{205f}DA\u{3000}B\u{3000}C\u{3000}D",
            "A\u{202f}B\u{202f}C\nDA\u{205f}\u{205f}B\u{205f}C\nDA\u{3000}B\nC\u{3000}D",
            7,
        ),
    ];

    #[test]
    fn test_wrap_cases() {
        for (name, input, expected, width) in WRAP_CASES {
            let got = wrap(input, *width, "");
            assert_eq!(got, *expected, "wrap case {name}: {input:?}");
        }
    }

    #[test]
    fn test_paragraph_separator_width() {
        // Upstream: ansi considers U+2029 zero width.
        assert_eq!(string_width("0\u{2029}1"), 2);
    }

    #[test]
    fn test_wrap_wide() {
        // Wide-char variant shares the same implementation.
        assert_eq!(wrap_wc("foo bar", 5, ""), "foo\nbar");
        assert_eq!(hardwrap_wc("foobarfoo", 4, true), "foob\narfo\no");
        assert_eq!(wrap_wc("こんにち", 7, ""), "こんに\nち");
    }
}
