//! Cleanroom Rust port of upstream Go source file: `ansi/kitty.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! Kitty keyboard protocol progressive enhancement flags and sequences.
//! </public-docs>

/// Disambiguate escape codes: `1`.
pub const KITTY_DISAMBIGUATE_ESCAPE_CODES: u8 = 1 << 0;

/// Report event types: `2`.
pub const KITTY_REPORT_EVENT_TYPES: u8 = 1 << 1;

/// Report alternate keys: `4`.
pub const KITTY_REPORT_ALTERNATE_KEYS: u8 = 1 << 2;

/// Report all keys as escape codes: `8`.
pub const KITTY_REPORT_ALL_KEYS_AS_ESCAPE_CODES: u8 = 1 << 3;

/// Report associated keys: `16`.
pub const KITTY_REPORT_ASSOCIATED_KEYS: u8 = 1 << 4;

/// All Kitty keyboard protocol flags: `31`.
pub const KITTY_ALL_FLAGS: u8 = KITTY_DISAMBIGUATE_ESCAPE_CODES
    | KITTY_REPORT_EVENT_TYPES
    | KITTY_REPORT_ALTERNATE_KEYS
    | KITTY_REPORT_ALL_KEYS_AS_ESCAPE_CODES
    | KITTY_REPORT_ASSOCIATED_KEYS;

/// RequestKittyKeyboard is a sequence to request the terminal Kitty keyboard
/// protocol enabled flags.
///
/// See: <https://sw.kovidgoyal.net/kitty/keyboard-protocol/>
pub const REQUEST_KITTY_KEYBOARD: &str = "\x1b[?u";

/// KittyKeyboard returns a sequence to request keyboard enhancements from the
/// terminal. The flags argument is a bitmask of the Kitty keyboard protocol
/// flags, while mode specifies how the flags should be interpreted.
///
/// Possible values for flags mask:
///
/// 1: Disambiguate escape codes
/// 2: Report event types
/// 4: Report alternate keys
/// 8: Report all keys as escape codes
/// 16: Report associated text
///
/// Possible values for mode:
///
/// 1: Set given flags and unset all others
/// 2: Set given flags and keep existing flags unchanged
/// 3: Unset given flags and keep existing flags unchanged
///
/// See: <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement>
pub fn kitty_keyboard(flags: u8, mode: u8) -> String {
    format!("\x1b[={};{}u", flags, mode)
}

/// PushKittyKeyboard returns a sequence to push the given flags to the
/// terminal Kitty Keyboard stack.
///
/// Possible values for flags mask:
///
/// 0: Disable all features
/// 1: Disambiguate escape codes
/// 2: Report event types
/// 4: Report alternate keys
/// 8: Report all keys as escape codes
/// 16: Report associated text
///
/// CSI > flags u
///
/// See: <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement>
pub fn push_kitty_keyboard(flags: u8) -> String {
    let f = if flags > 0 {
        flags.to_string()
    } else {
        String::new()
    };
    format!("\x1b[>{}u", f)
}

/// DisableKittyKeyboard is a sequence to push zero into the terminal Kitty
/// Keyboard stack to disable the protocol.
///
/// This is equivalent to PushKittyKeyboard(0).
pub const DISABLE_KITTY_KEYBOARD: &str = "\x1b[>u";

/// PopKittyKeyboard returns a sequence to pop n number of flags from the
/// terminal Kitty Keyboard stack.
///
/// CSI < flags u
///
/// See: <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement>
pub fn pop_kitty_keyboard(n: u8) -> String {
    let num = if n > 0 { n.to_string() } else { String::new() };
    format!("\x1b[<{}u", num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_values() {
        assert_eq!(KITTY_DISAMBIGUATE_ESCAPE_CODES, 1);
        assert_eq!(KITTY_REPORT_EVENT_TYPES, 2);
        assert_eq!(KITTY_REPORT_ALTERNATE_KEYS, 4);
        assert_eq!(KITTY_REPORT_ALL_KEYS_AS_ESCAPE_CODES, 8);
        assert_eq!(KITTY_REPORT_ASSOCIATED_KEYS, 16);
        assert_eq!(KITTY_ALL_FLAGS, 31);
    }

    #[test]
    fn test_sequences() {
        assert_eq!(REQUEST_KITTY_KEYBOARD, "\x1b[?u");
        assert_eq!(kitty_keyboard(1, 2), "\x1b[=1;2u");
        assert_eq!(kitty_keyboard(31, 1), "\x1b[=31;1u");
        assert_eq!(push_kitty_keyboard(0), "\x1b[>u");
        assert_eq!(push_kitty_keyboard(3), "\x1b[>3u");
        assert_eq!(DISABLE_KITTY_KEYBOARD, "\x1b[>u");
        assert_eq!(pop_kitty_keyboard(0), "\x1b[<u");
        assert_eq!(pop_kitty_keyboard(2), "\x1b[<2u");
    }
}
