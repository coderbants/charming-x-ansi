//! Cleanroom Rust port of upstream Go source file: `ansi/hyperlink.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! OSC 8 hyperlink sequences.
//! </public-docs>

/// Sets an OSC 8 hyperlink with the given URL and optional parameters.
pub fn set_hyperlink(link: &str, params: &str) -> String {
    if params.is_empty() {
        format!("\x1b]8;;{}\x07", link)
    } else {
        format!("\x1b]8;{};{}\x07", params, link)
    }
}

/// Returns the OSC 8 hyperlink reset sequence.
pub fn reset_hyperlink() -> &'static str {
    "\x1b]8;;\x07"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_hyperlink() {
        assert_eq!(set_hyperlink("https://example.com", ""), "\x1b]8;;https://example.com\x07");
        assert_eq!(
            set_hyperlink("https://example.com", "id=1"),
            "\x1b]8;id=1;https://example.com\x07"
        );
    }
}
