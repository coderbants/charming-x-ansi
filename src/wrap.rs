//! Cleanroom Rust port of upstream Go source file: `ansi/wrap.go`
//! Upstream Target Tag / Version: `v0.11.2`
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
pub fn wrap(s: &str, limit: usize, breakpoints: &str) -> String {
    wordwrap(s, limit, breakpoints)
}

/// Wraps with the wide-character width method.
pub fn wrap_wc(s: &str, limit: usize, breakpoints: &str) -> String {
    wordwrap(s, limit, breakpoints)
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

fn hardwrap_impl(s: &str, limit: usize, preserve_space: bool) -> String {
    if limit < 1 {
        return s.to_string();
    }
    let mut buf = String::new();
    let mut cur_width = 0usize;
    let mut force_newline = false;

    for g in unicode_segmentation::UnicodeSegmentation::graphemes(s, true) {
        if g.starts_with('\x1b') {
            buf.push_str(g);
            continue;
        }
        let r = g.chars().next().unwrap();
        let w = string_width(g);
        if r == '\n' {
            buf.push('\n');
            cur_width = 0;
            force_newline = false;
            continue;
        }
        if cur_width + w > limit {
            buf.push('\n');
            cur_width = 0;
            force_newline = true;
        }
        if cur_width == 0 {
            if !preserve_space && force_newline && r.is_whitespace() {
                continue;
            }
            force_newline = false;
        }
        buf.push_str(g);
        cur_width += w;
    }
    buf
}

#[allow(unused_assignments)]
fn wordwrap(s: &str, limit: usize, breakpoints: &str) -> String {
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

    // Tokenize the input into graphemes and ANSI escape sequences.
    let mut tokens: Vec<String> = Vec::new();
    let mut rest = s;
    while !rest.is_empty() {
        if rest.starts_with('\x1b') {
            let (seq, len) = take_escape(rest);
            tokens.push(seq);
            rest = &rest[len..];
        } else {
            let g = unicode_segmentation::UnicodeSegmentation::graphemes(rest, true)
                .next()
                .unwrap();
            tokens.push(g.to_string());
            rest = &rest[g.len()..];
        }
    }

    for token in &tokens {
        if token.starts_with('\x1b') {
            word.push_str(token);
            continue;
        }
        let r = token.chars().next().unwrap();
        let w = string_width(token);
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
            space.push_str(token);
            space_width += w;
        } else if breakpoints.contains(&r) || r == '-' {
            add_space!();
            if cur_width + word_len + w > limit {
                word.push_str(token);
                word_len += w;
            } else {
                add_word!();
                buf.push_str(token);
                cur_width += w;
            }
        } else {
            if word_len + w > limit {
                add_word!();
            }
            word.push_str(token);
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
/// the sequence and its byte length.
fn take_escape(s: &str) -> (String, usize) {
    let chars: Vec<char> = s.chars().collect();
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
    let seq: String = chars[..consumed.min(chars.len())].iter().collect();
    let len = seq.len();
    (seq, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_basic() {
        assert_eq!(wrap("The quick brown fox jumps over the lazy dog", 10, ""),
            "The quick\nbrown fox\njumps over\nthe lazy\ndog");
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
}
