//! Cleanroom Rust port of upstream Go source file: `ansi/util.go` and `ansi/width.go`
//! Upstream Target Tag / Version: `v0.11.2`
//!
//! <public-docs>
//! ANSI-aware string utilities: stripping, cutting, truncation, and cell-width
//! measurement.
//! </public-docs>

use unicode_width::UnicodeWidthChar;

/// StringWidth returns the width of the string in cells, ignoring ANSI
/// sequences and measuring wide characters correctly.
pub fn string_width(s: &str) -> usize {
    let s = strip(s);
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap_or(0)
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
        if width >= end {
            break;
        }
        if width >= start {
            out.push(c);
        }
        width += w;
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
}
