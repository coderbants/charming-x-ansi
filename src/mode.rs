//! Cleanroom Rust port of upstream Go source file: `ansi/mode.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! Also ports `ansi/mode_deprecated.go` (deprecated mode aliases) and
//! `ansi/modes.go` (the [`Modes`] state map).
//!
//! <public-docs>
//! Terminal mode constants and helpers: SM/DECSET (`h`), RM/DECRST (`l`),
//! DECRQM (`$p`) and DECRPM (`$y`) sequences for ANSI and DEC private modes.
//! </public-docs>

use std::collections::HashMap;

/// ModeSetting represents a mode setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModeSetting(pub u8);

impl ModeSetting {
    /// The mode is not recognized.
    pub const NOT_RECOGNIZED: ModeSetting = ModeSetting(0);
    /// The mode is set.
    pub const SET: ModeSetting = ModeSetting(1);
    /// The mode is reset.
    pub const RESET: ModeSetting = ModeSetting(2);
    /// The mode is permanently set.
    pub const PERMANENTLY_SET: ModeSetting = ModeSetting(3);
    /// The mode is permanently reset.
    pub const PERMANENTLY_RESET: ModeSetting = ModeSetting(4);

    /// IsNotRecognized returns true if the mode is not recognized.
    pub fn is_not_recognized(self) -> bool {
        self == ModeSetting::NOT_RECOGNIZED
    }

    /// IsSet returns true if the mode is set or permanently set.
    pub fn is_set(self) -> bool {
        self == ModeSetting::SET || self == ModeSetting::PERMANENTLY_SET
    }

    /// IsReset returns true if the mode is reset or permanently reset.
    pub fn is_reset(self) -> bool {
        self == ModeSetting::RESET || self == ModeSetting::PERMANENTLY_RESET
    }

    /// IsPermanentlySet returns true if the mode is permanently set.
    pub fn is_permanently_set(self) -> bool {
        self == ModeSetting::PERMANENTLY_SET
    }

    /// IsPermanentlyReset returns true if the mode is permanently reset.
    pub fn is_permanently_reset(self) -> bool {
        self == ModeSetting::PERMANENTLY_RESET
    }
}

/// ANSIMode represents an ANSI terminal mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ANSIMode(pub i32);

/// DECMode represents a private DEC terminal mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DECMode(pub i32);

/// Mode represents a terminal mode: an ANSI mode or a private DEC mode.
///
/// Ports the upstream `Mode` interface (implemented by both `ANSIMode` and
/// `DECMode`) as a closed enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    /// An ANSI mode.
    Ansi(ANSIMode),
    /// A private DEC mode.
    Dec(DECMode),
}

impl Mode {
    /// Mode returns the mode as an integer.
    pub fn mode(self) -> i32 {
        match self {
            Mode::Ansi(m) => m.0,
            Mode::Dec(m) => m.0,
        }
    }

    /// Returns true if this is a private DEC mode.
    pub fn is_dec(self) -> bool {
        matches!(self, Mode::Dec(_))
    }
}

impl From<ANSIMode> for Mode {
    fn from(m: ANSIMode) -> Mode {
        Mode::Ansi(m)
    }
}

impl From<DECMode> for Mode {
    fn from(m: DECMode) -> Mode {
        Mode::Dec(m)
    }
}

impl ANSIMode {
    /// Mode returns the ANSI mode as an integer.
    pub fn mode(self) -> i32 {
        self.0
    }
}

impl DECMode {
    /// Mode returns the DEC mode as an integer.
    pub fn mode(self) -> i32 {
        self.0
    }
}

/// SetMode (SM) or (DECSET) returns a sequence to set a mode.
/// The mode arguments are a list of modes to set.
///
/// If one of the modes is a [DECMode], the function returns two escape
/// sequences.
///
/// ANSI format: `CSI Pd ; ... ; Pd h`
///
/// DEC format: `CSI ? Pd ; ... ; Pd h`
///
/// See: <https://vt100.net/docs/vt510-rm/SM.html>
pub fn set_mode(modes: &[Mode]) -> String {
    set_mode_impl(false, modes)
}

/// SM is an alias for [set_mode].
pub fn sm(modes: &[Mode]) -> String {
    set_mode(modes)
}

/// DECSET is an alias for [set_mode].
pub fn decset(modes: &[Mode]) -> String {
    set_mode(modes)
}

/// ResetMode (RM) or (DECRST) returns a sequence to reset a mode.
/// The mode arguments are a list of modes to reset.
///
/// If one of the modes is a [DECMode], the function returns two escape
/// sequences.
///
/// ANSI format: `CSI Pd ; ... ; Pd l`
///
/// DEC format: `CSI ? Pd ; ... ; Pd l`
///
/// See: <https://vt100.net/docs/vt510-rm/RM.html>
pub fn reset_mode(modes: &[Mode]) -> String {
    set_mode_impl(true, modes)
}

/// RM is an alias for [reset_mode].
pub fn rm(modes: &[Mode]) -> String {
    reset_mode(modes)
}

/// DECRST is an alias for [reset_mode].
pub fn decrst(modes: &[Mode]) -> String {
    reset_mode(modes)
}

fn set_mode_impl(reset: bool, modes: &[Mode]) -> String {
    if modes.is_empty() {
        return String::new();
    }

    let cmd = if reset { "l" } else { "h" };

    let mut seq = String::from("\x1b[");
    if modes.len() == 1 {
        if modes[0].is_dec() {
            seq.push('?');
        }
        seq.push_str(&modes[0].mode().to_string());
        seq.push_str(cmd);
        return seq;
    }

    let mut dec: Vec<String> = Vec::new();
    let mut ansi: Vec<String> = Vec::new();
    for m in modes {
        match m {
            Mode::Dec(d) => dec.push(d.0.to_string()),
            Mode::Ansi(a) => ansi.push(a.0.to_string()),
        }
    }

    let mut s = String::new();
    if !ansi.is_empty() {
        s.push_str(&seq);
        s.push_str(&ansi.join(";"));
        s.push_str(cmd);
    }
    if !dec.is_empty() {
        s.push_str("\x1b[?");
        s.push_str(&dec.join(";"));
        s.push_str(cmd);
    }
    s
}

/// RequestMode (DECRQM) returns a sequence to request a mode from the
/// terminal. The terminal responds with a report mode function [report_mode].
///
/// ANSI format: `CSI Pa $ p`
///
/// DEC format: `CSI ? Pa $ p`
///
/// See: <https://vt100.net/docs/vt510-rm/DECRQM.html>
pub fn request_mode(m: Mode) -> String {
    let mut seq = String::from("\x1b[");
    if m.is_dec() {
        seq.push('?');
    }
    seq.push_str(&m.mode().to_string());
    seq.push_str("$p");
    seq
}

/// DECRQM is an alias for [request_mode].
pub fn decrqm(m: Mode) -> String {
    request_mode(m)
}

/// ReportMode (DECRPM) returns a sequence that the terminal sends to the host
/// in response to a mode request [request_mode].
///
/// ANSI format: `CSI Pa ; Ps ; $ y`
///
/// DEC format: `CSI ? Pa ; Ps $ y`
///
/// Where Pa is the mode number, and Ps is the mode value:
///
/// 0: Not recognized, 1: Set, 2: Reset, 3: Permanent set, 4: Permanent reset.
///
/// See: <https://vt100.net/docs/vt510-rm/DECRPM.html>
pub fn report_mode(mode: Mode, value: ModeSetting) -> String {
    let v = if value.0 > 4 { 0 } else { value.0 };
    if mode.is_dec() {
        return format!("\x1b[?{};{}$y", mode.mode(), v);
    }
    format!("\x1b[{};{}$y", mode.mode(), v)
}

/// DECRPM is an alias for [report_mode].
pub fn decrpm(mode: Mode, value: ModeSetting) -> String {
    report_mode(mode, value)
}

// ---------------------------------------------------------------------------
// Mode constants (ports `ansi/mode.go`).
// ---------------------------------------------------------------------------

/// Keyboard Action Mode (KAM): controls locking of the keyboard. `2`.
///
/// See: <https://vt100.net/docs/vt510-rm/KAM.html>
pub const MODE_KEYBOARD_ACTION: ANSIMode = ANSIMode(2);
/// Alias for [MODE_KEYBOARD_ACTION].
pub const KAM: ANSIMode = MODE_KEYBOARD_ACTION;
/// Set sequence for [MODE_KEYBOARD_ACTION].
pub const SET_MODE_KEYBOARD_ACTION: &str = "\x1b[2h";
/// Reset sequence for [MODE_KEYBOARD_ACTION].
pub const RESET_MODE_KEYBOARD_ACTION: &str = "\x1b[2l";
/// Request sequence for [MODE_KEYBOARD_ACTION].
pub const REQUEST_MODE_KEYBOARD_ACTION: &str = "\x1b[2$p";

/// Insert/Replace Mode (IRM): determines whether characters are inserted or
/// replaced when typed. `4`.
///
/// See: <https://vt100.net/docs/vt510-rm/IRM.html>
pub const MODE_INSERT_REPLACE: ANSIMode = ANSIMode(4);
/// Alias for [MODE_INSERT_REPLACE].
pub const IRM: ANSIMode = MODE_INSERT_REPLACE;
/// Set sequence for [MODE_INSERT_REPLACE].
pub const SET_MODE_INSERT_REPLACE: &str = "\x1b[4h";
/// Reset sequence for [MODE_INSERT_REPLACE].
pub const RESET_MODE_INSERT_REPLACE: &str = "\x1b[4l";
/// Request sequence for [MODE_INSERT_REPLACE].
pub const REQUEST_MODE_INSERT_REPLACE: &str = "\x1b[4$p";

/// BiDirectional Support Mode (BDSM): determines whether the terminal supports
/// bidirectional text. `8`. See ECMA-48 7.2.1.
pub const MODE_BI_DIRECTIONAL_SUPPORT: ANSIMode = ANSIMode(8);
/// Alias for [MODE_BI_DIRECTIONAL_SUPPORT].
pub const BDSM: ANSIMode = MODE_BI_DIRECTIONAL_SUPPORT;
/// Set sequence for [MODE_BI_DIRECTIONAL_SUPPORT].
pub const SET_MODE_BI_DIRECTIONAL_SUPPORT: &str = "\x1b[8h";
/// Reset sequence for [MODE_BI_DIRECTIONAL_SUPPORT].
pub const RESET_MODE_BI_DIRECTIONAL_SUPPORT: &str = "\x1b[8l";
/// Request sequence for [MODE_BI_DIRECTIONAL_SUPPORT].
pub const REQUEST_MODE_BI_DIRECTIONAL_SUPPORT: &str = "\x1b[8$p";

/// Send Receive Mode (SRM) or Local Echo Mode: determines whether the terminal
/// echoes characters back to the host. `12`.
///
/// See: <https://vt100.net/docs/vt510-rm/SRM.html>
pub const MODE_SEND_RECEIVE: ANSIMode = ANSIMode(12);
/// Alias for [MODE_SEND_RECEIVE].
pub const MODE_LOCAL_ECHO: ANSIMode = MODE_SEND_RECEIVE;
/// Alias for [MODE_SEND_RECEIVE].
pub const SRM: ANSIMode = MODE_SEND_RECEIVE;
/// Set sequence for [MODE_SEND_RECEIVE].
pub const SET_MODE_SEND_RECEIVE: &str = "\x1b[12h";
/// Reset sequence for [MODE_SEND_RECEIVE].
pub const RESET_MODE_SEND_RECEIVE: &str = "\x1b[12l";
/// Request sequence for [MODE_SEND_RECEIVE].
pub const REQUEST_MODE_SEND_RECEIVE: &str = "\x1b[12$p";
/// Set sequence for [MODE_LOCAL_ECHO].
pub const SET_MODE_LOCAL_ECHO: &str = "\x1b[12h";
/// Reset sequence for [MODE_LOCAL_ECHO].
pub const RESET_MODE_LOCAL_ECHO: &str = "\x1b[12l";
/// Request sequence for [MODE_LOCAL_ECHO].
pub const REQUEST_MODE_LOCAL_ECHO: &str = "\x1b[12$p";

/// Line Feed/New Line Mode (LNM): determines whether the terminal interprets
/// the line feed character as a new line. `20`.
///
/// See: <https://vt100.net/docs/vt510-rm/LNM.html>
pub const MODE_LINE_FEED_NEW_LINE: ANSIMode = ANSIMode(20);
/// Alias for [MODE_LINE_FEED_NEW_LINE].
pub const LNM: ANSIMode = MODE_LINE_FEED_NEW_LINE;
/// Set sequence for [MODE_LINE_FEED_NEW_LINE].
pub const SET_MODE_LINE_FEED_NEW_LINE: &str = "\x1b[20h";
/// Reset sequence for [MODE_LINE_FEED_NEW_LINE].
pub const RESET_MODE_LINE_FEED_NEW_LINE: &str = "\x1b[20l";
/// Request sequence for [MODE_LINE_FEED_NEW_LINE].
pub const REQUEST_MODE_LINE_FEED_NEW_LINE: &str = "\x1b[20$p";

/// Cursor Keys Mode (DECCKM): determines whether the cursor keys send ANSI
/// cursor sequences or application sequences. `1`.
///
/// See: <https://vt100.net/docs/vt510-rm/DECCKM.html>
pub const MODE_CURSOR_KEYS: DECMode = DECMode(1);
/// Alias for [MODE_CURSOR_KEYS].
pub const DECCKM: DECMode = MODE_CURSOR_KEYS;
/// Set sequence for [MODE_CURSOR_KEYS].
pub const SET_MODE_CURSOR_KEYS: &str = "\x1b[?1h";
/// Reset sequence for [MODE_CURSOR_KEYS].
pub const RESET_MODE_CURSOR_KEYS: &str = "\x1b[?1l";
/// Request sequence for [MODE_CURSOR_KEYS].
pub const REQUEST_MODE_CURSOR_KEYS: &str = "\x1b[?1$p";

/// Origin Mode (DECOM): determines whether the cursor moves to the home
/// position or the margin position. `6`.
///
/// See: <https://vt100.net/docs/vt510-rm/DECOM.html>
pub const MODE_ORIGIN: DECMode = DECMode(6);
/// Alias for [MODE_ORIGIN].
pub const DECOM: DECMode = MODE_ORIGIN;
/// Set sequence for [MODE_ORIGIN].
pub const SET_MODE_ORIGIN: &str = "\x1b[?6h";
/// Reset sequence for [MODE_ORIGIN].
pub const RESET_MODE_ORIGIN: &str = "\x1b[?6l";
/// Request sequence for [MODE_ORIGIN].
pub const REQUEST_MODE_ORIGIN: &str = "\x1b[?6$p";

/// Auto Wrap Mode (DECAWM): determines whether the cursor wraps to the next
/// line when it reaches the right margin. `7`.
///
/// See: <https://vt100.net/docs/vt510-rm/DECAWM.html>
pub const MODE_AUTO_WRAP: DECMode = DECMode(7);
/// Alias for [MODE_AUTO_WRAP].
pub const DECAWM: DECMode = MODE_AUTO_WRAP;
/// Set sequence for [MODE_AUTO_WRAP].
pub const SET_MODE_AUTO_WRAP: &str = "\x1b[?7h";
/// Reset sequence for [MODE_AUTO_WRAP].
pub const RESET_MODE_AUTO_WRAP: &str = "\x1b[?7l";
/// Request sequence for [MODE_AUTO_WRAP].
pub const REQUEST_MODE_AUTO_WRAP: &str = "\x1b[?7$p";

/// X10 Mouse Mode: determines whether the mouse reports on button presses.
///
/// The terminal responds with `CSI M CbCxCy`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_X10: DECMode = DECMode(9);
/// Set sequence for [MODE_MOUSE_X10].
pub const SET_MODE_MOUSE_X10: &str = "\x1b[?9h";
/// Reset sequence for [MODE_MOUSE_X10].
pub const RESET_MODE_MOUSE_X10: &str = "\x1b[?9l";
/// Request sequence for [MODE_MOUSE_X10].
pub const REQUEST_MODE_MOUSE_X10: &str = "\x1b[?9$p";

/// Text Cursor Enable Mode (DECTCEM): shows/hides the cursor. `25`.
///
/// See: <https://vt100.net/docs/vt510-rm/DECTCEM.html>
pub const MODE_TEXT_CURSOR_ENABLE: DECMode = DECMode(25);
/// Alias for [MODE_TEXT_CURSOR_ENABLE].
pub const DECTCEM: DECMode = MODE_TEXT_CURSOR_ENABLE;
/// Set sequence for [MODE_TEXT_CURSOR_ENABLE].
pub const SET_MODE_TEXT_CURSOR_ENABLE: &str = "\x1b[?25h";
/// Reset sequence for [MODE_TEXT_CURSOR_ENABLE].
pub const RESET_MODE_TEXT_CURSOR_ENABLE: &str = "\x1b[?25l";
/// Request sequence for [MODE_TEXT_CURSOR_ENABLE].
pub const REQUEST_MODE_TEXT_CURSOR_ENABLE: &str = "\x1b[?25$p";

/// Alias for [SET_MODE_TEXT_CURSOR_ENABLE].
pub const SHOW_CURSOR: &str = SET_MODE_TEXT_CURSOR_ENABLE;
/// Alias for [RESET_MODE_TEXT_CURSOR_ENABLE].
pub const HIDE_CURSOR: &str = RESET_MODE_TEXT_CURSOR_ENABLE;

/// Numeric Keypad Mode (DECNKM): determines whether the keypad sends
/// application sequences or numeric sequences. `66`.
///
/// See: <https://vt100.net/docs/vt510-rm/DECNKM.html>
pub const MODE_NUMERIC_KEYPAD: DECMode = DECMode(66);
/// Alias for [MODE_NUMERIC_KEYPAD].
pub const DECNKM: DECMode = MODE_NUMERIC_KEYPAD;
/// Set sequence for [MODE_NUMERIC_KEYPAD].
pub const SET_MODE_NUMERIC_KEYPAD: &str = "\x1b[?66h";
/// Reset sequence for [MODE_NUMERIC_KEYPAD].
pub const RESET_MODE_NUMERIC_KEYPAD: &str = "\x1b[?66l";
/// Request sequence for [MODE_NUMERIC_KEYPAD].
pub const REQUEST_MODE_NUMERIC_KEYPAD: &str = "\x1b[?66$p";

/// Backarrow Key Mode (DECBKM): determines whether the backspace key sends a
/// backspace or delete character. Disabled by default. `67`.
///
/// See: <https://vt100.net/docs/vt510-rm/DECBKM.html>
pub const MODE_BACKARROW_KEY: DECMode = DECMode(67);
/// Alias for [MODE_BACKARROW_KEY].
pub const DECBKM: DECMode = MODE_BACKARROW_KEY;
/// Set sequence for [MODE_BACKARROW_KEY].
pub const SET_MODE_BACKARROW_KEY: &str = "\x1b[?67h";
/// Reset sequence for [MODE_BACKARROW_KEY].
pub const RESET_MODE_BACKARROW_KEY: &str = "\x1b[?67l";
/// Request sequence for [MODE_BACKARROW_KEY].
pub const REQUEST_MODE_BACKARROW_KEY: &str = "\x1b[?67$p";

/// Left Right Margin Mode (DECLRMM): determines whether the left and right
/// margins can be set with DECSLRM. `69`.
///
/// See: <https://vt100.net/docs/vt510-rm/DECLRMM.html>
pub const MODE_LEFT_RIGHT_MARGIN: DECMode = DECMode(69);
/// Alias for [MODE_LEFT_RIGHT_MARGIN].
pub const DECLRMM: DECMode = MODE_LEFT_RIGHT_MARGIN;
/// Set sequence for [MODE_LEFT_RIGHT_MARGIN].
pub const SET_MODE_LEFT_RIGHT_MARGIN: &str = "\x1b[?69h";
/// Reset sequence for [MODE_LEFT_RIGHT_MARGIN].
pub const RESET_MODE_LEFT_RIGHT_MARGIN: &str = "\x1b[?69l";
/// Request sequence for [MODE_LEFT_RIGHT_MARGIN].
pub const REQUEST_MODE_LEFT_RIGHT_MARGIN: &str = "\x1b[?69$p";

/// Normal Mouse Mode: determines whether the mouse reports on button presses
/// and releases. It will also report modifier keys, wheel events, and extra
/// buttons. `1000`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_NORMAL: DECMode = DECMode(1000);
/// Set sequence for [MODE_MOUSE_NORMAL].
pub const SET_MODE_MOUSE_NORMAL: &str = "\x1b[?1000h";
/// Reset sequence for [MODE_MOUSE_NORMAL].
pub const RESET_MODE_MOUSE_NORMAL: &str = "\x1b[?1000l";
/// Request sequence for [MODE_MOUSE_NORMAL].
pub const REQUEST_MODE_MOUSE_NORMAL: &str = "\x1b[?1000$p";

/// Highlight Mouse Tracking: determines whether the mouse reports on button
/// presses, releases, and highlighted cells. `1001`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_HIGHLIGHT: DECMode = DECMode(1001);
/// Set sequence for [MODE_MOUSE_HIGHLIGHT].
pub const SET_MODE_MOUSE_HIGHLIGHT: &str = "\x1b[?1001h";
/// Reset sequence for [MODE_MOUSE_HIGHLIGHT].
pub const RESET_MODE_MOUSE_HIGHLIGHT: &str = "\x1b[?1001l";
/// Request sequence for [MODE_MOUSE_HIGHLIGHT].
pub const REQUEST_MODE_MOUSE_HIGHLIGHT: &str = "\x1b[?1001$p";

/// Button Event Mouse Tracking: essentially the same as [MODE_MOUSE_NORMAL],
/// but it also reports button-motion events when a button is pressed. `1002`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_BUTTON_EVENT: DECMode = DECMode(1002);
/// Set sequence for [MODE_MOUSE_BUTTON_EVENT].
pub const SET_MODE_MOUSE_BUTTON_EVENT: &str = "\x1b[?1002h";
/// Reset sequence for [MODE_MOUSE_BUTTON_EVENT].
pub const RESET_MODE_MOUSE_BUTTON_EVENT: &str = "\x1b[?1002l";
/// Request sequence for [MODE_MOUSE_BUTTON_EVENT].
pub const REQUEST_MODE_MOUSE_BUTTON_EVENT: &str = "\x1b[?1002$p";

/// Any Event Mouse Tracking: the same as [MODE_MOUSE_BUTTON_EVENT], except
/// that all motion events are reported even if no mouse buttons are pressed.
/// `1003`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_ANY_EVENT: DECMode = DECMode(1003);
/// Set sequence for [MODE_MOUSE_ANY_EVENT].
pub const SET_MODE_MOUSE_ANY_EVENT: &str = "\x1b[?1003h";
/// Reset sequence for [MODE_MOUSE_ANY_EVENT].
pub const RESET_MODE_MOUSE_ANY_EVENT: &str = "\x1b[?1003l";
/// Request sequence for [MODE_MOUSE_ANY_EVENT].
pub const REQUEST_MODE_MOUSE_ANY_EVENT: &str = "\x1b[?1003$p";

/// Focus Event Mode: determines whether the terminal reports focus and blur
/// events. `1004`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Focus-Tracking>
pub const MODE_FOCUS_EVENT: DECMode = DECMode(1004);
/// Set sequence for [MODE_FOCUS_EVENT].
pub const SET_MODE_FOCUS_EVENT: &str = "\x1b[?1004h";
/// Reset sequence for [MODE_FOCUS_EVENT].
pub const RESET_MODE_FOCUS_EVENT: &str = "\x1b[?1004l";
/// Request sequence for [MODE_FOCUS_EVENT].
pub const REQUEST_MODE_FOCUS_EVENT: &str = "\x1b[?1004$p";

/// SGR Extended Mouse Mode: changes the mouse tracking encoding to use SGR
/// parameters. The terminal responds with `CSI < Cb ; Cx ; Cy M`. `1006`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_EXT_SGR: DECMode = DECMode(1006);
/// Set sequence for [MODE_MOUSE_EXT_SGR].
pub const SET_MODE_MOUSE_EXT_SGR: &str = "\x1b[?1006h";
/// Reset sequence for [MODE_MOUSE_EXT_SGR].
pub const RESET_MODE_MOUSE_EXT_SGR: &str = "\x1b[?1006l";
/// Request sequence for [MODE_MOUSE_EXT_SGR].
pub const REQUEST_MODE_MOUSE_EXT_SGR: &str = "\x1b[?1006$p";

/// UTF-8 Extended Mouse Mode: changes the mouse tracking encoding to use UTF-8
/// parameters. `1005`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_EXT_UTF8: DECMode = DECMode(1005);
/// Set sequence for [MODE_MOUSE_EXT_UTF8].
pub const SET_MODE_MOUSE_EXT_UTF8: &str = "\x1b[?1005h";
/// Reset sequence for [MODE_MOUSE_EXT_UTF8].
pub const RESET_MODE_MOUSE_EXT_UTF8: &str = "\x1b[?1005l";
/// Request sequence for [MODE_MOUSE_EXT_UTF8].
pub const REQUEST_MODE_MOUSE_EXT_UTF8: &str = "\x1b[?1005$p";

/// URXVT Extended Mouse Mode: changes the mouse tracking encoding to use an
/// alternate encoding. `1015`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_EXT_URXVT: DECMode = DECMode(1015);
/// Set sequence for [MODE_MOUSE_EXT_URXVT].
pub const SET_MODE_MOUSE_EXT_URXVT: &str = "\x1b[?1015h";
/// Reset sequence for [MODE_MOUSE_EXT_URXVT].
pub const RESET_MODE_MOUSE_EXT_URXVT: &str = "\x1b[?1015l";
/// Request sequence for [MODE_MOUSE_EXT_URXVT].
pub const REQUEST_MODE_MOUSE_EXT_URXVT: &str = "\x1b[?1015$p";

/// SGR Pixel Extended Mouse Mode: changes the mouse tracking encoding to use
/// SGR parameters with pixel coordinates. Similar to [MODE_MOUSE_EXT_SGR].
/// `1016`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Mouse-Tracking>
pub const MODE_MOUSE_EXT_SGR_PIXEL: DECMode = DECMode(1016);
/// Set sequence for [MODE_MOUSE_EXT_SGR_PIXEL].
pub const SET_MODE_MOUSE_EXT_SGR_PIXEL: &str = "\x1b[?1016h";
/// Reset sequence for [MODE_MOUSE_EXT_SGR_PIXEL].
pub const RESET_MODE_MOUSE_EXT_SGR_PIXEL: &str = "\x1b[?1016l";
/// Request sequence for [MODE_MOUSE_EXT_SGR_PIXEL].
pub const REQUEST_MODE_MOUSE_EXT_SGR_PIXEL: &str = "\x1b[?1016$p";

/// Alternate Screen Mode: determines whether the alternate screen buffer is
/// active. When enabled, the alternate screen buffer is cleared. `1047`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-The-Alternate-Screen-Buffer>
pub const MODE_ALT_SCREEN: DECMode = DECMode(1047);
/// Set sequence for [MODE_ALT_SCREEN].
pub const SET_MODE_ALT_SCREEN: &str = "\x1b[?1047h";
/// Reset sequence for [MODE_ALT_SCREEN].
pub const RESET_MODE_ALT_SCREEN: &str = "\x1b[?1047l";
/// Request sequence for [MODE_ALT_SCREEN].
pub const REQUEST_MODE_ALT_SCREEN: &str = "\x1b[?1047$p";

/// Save Cursor Mode: saves the cursor position. Equivalent to [crate::cursor::SAVE_CURSOR]
/// and [crate::cursor::RESTORE_CURSOR]. `1048`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-The-Alternate-Screen-Buffer>
pub const MODE_SAVE_CURSOR: DECMode = DECMode(1048);
/// Set sequence for [MODE_SAVE_CURSOR].
pub const SET_MODE_SAVE_CURSOR: &str = "\x1b[?1048h";
/// Reset sequence for [MODE_SAVE_CURSOR].
pub const RESET_MODE_SAVE_CURSOR: &str = "\x1b[?1048l";
/// Request sequence for [MODE_SAVE_CURSOR].
pub const REQUEST_MODE_SAVE_CURSOR: &str = "\x1b[?1048$p";

/// Alternate Screen Save Cursor Mode: saves the cursor position as in
/// [MODE_SAVE_CURSOR], switches to the alternate screen buffer as in
/// [MODE_ALT_SCREEN], and clears the screen on switch. `1049`.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-The-Alternate-Screen-Buffer>
pub const MODE_ALT_SCREEN_SAVE_CURSOR: DECMode = DECMode(1049);
/// Set sequence for [MODE_ALT_SCREEN_SAVE_CURSOR].
pub const SET_MODE_ALT_SCREEN_SAVE_CURSOR: &str = "\x1b[?1049h";
/// Reset sequence for [MODE_ALT_SCREEN_SAVE_CURSOR].
pub const RESET_MODE_ALT_SCREEN_SAVE_CURSOR: &str = "\x1b[?1049l";
/// Request sequence for [MODE_ALT_SCREEN_SAVE_CURSOR].
pub const REQUEST_MODE_ALT_SCREEN_SAVE_CURSOR: &str = "\x1b[?1049$p";

/// Bracketed Paste Mode: determines whether pasted text is bracketed with
/// escape sequences. `2004`.
///
/// See: <https://cirw.in/blog/bracketed-paste>
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Bracketed-Paste-Mode>
pub const MODE_BRACKETED_PASTE: DECMode = DECMode(2004);
/// Set sequence for [MODE_BRACKETED_PASTE].
pub const SET_MODE_BRACKETED_PASTE: &str = "\x1b[?2004h";
/// Reset sequence for [MODE_BRACKETED_PASTE].
pub const RESET_MODE_BRACKETED_PASTE: &str = "\x1b[?2004l";
/// Request sequence for [MODE_BRACKETED_PASTE].
pub const REQUEST_MODE_BRACKETED_PASTE: &str = "\x1b[?2004$p";

/// Synchronized Output Mode: determines whether output is synchronized with
/// the terminal. `2026`.
///
/// See: <https://gist.github.com/christianparpart/d8a62cc1ab659194337d73e399004036>
pub const MODE_SYNCHRONIZED_OUTPUT: DECMode = DECMode(2026);
/// Set sequence for [MODE_SYNCHRONIZED_OUTPUT].
pub const SET_MODE_SYNCHRONIZED_OUTPUT: &str = "\x1b[?2026h";
/// Reset sequence for [MODE_SYNCHRONIZED_OUTPUT].
pub const RESET_MODE_SYNCHRONIZED_OUTPUT: &str = "\x1b[?2026l";
/// Request sequence for [MODE_SYNCHRONIZED_OUTPUT].
pub const REQUEST_MODE_SYNCHRONIZED_OUTPUT: &str = "\x1b[?2026$p";

/// Unicode Core Mode: determines whether the terminal should use Unicode
/// grapheme clustering to calculate the width of glyphs for each terminal
/// cell. `2027`.
///
/// See: <https://github.com/contour-terminal/terminal-unicode-core>
pub const MODE_UNICODE_CORE: DECMode = DECMode(2027);
/// Set sequence for [MODE_UNICODE_CORE].
pub const SET_MODE_UNICODE_CORE: &str = "\x1b[?2027h";
/// Reset sequence for [MODE_UNICODE_CORE].
pub const RESET_MODE_UNICODE_CORE: &str = "\x1b[?2027l";
/// Request sequence for [MODE_UNICODE_CORE].
pub const REQUEST_MODE_UNICODE_CORE: &str = "\x1b[?2027$p";

/// ModeLightDark: enables reporting the operating system's color scheme
/// (light or dark) preference. `2031`.
///
/// See: <https://contour-terminal.org/vt-extensions/color-palette-update-notifications/>
pub const MODE_LIGHT_DARK: DECMode = DECMode(2031);
/// Set sequence for [MODE_LIGHT_DARK].
pub const SET_MODE_LIGHT_DARK: &str = "\x1b[?2031h";
/// Reset sequence for [MODE_LIGHT_DARK].
pub const RESET_MODE_LIGHT_DARK: &str = "\x1b[?2031l";
/// Request sequence for [MODE_LIGHT_DARK].
pub const REQUEST_MODE_LIGHT_DARK: &str = "\x1b[?2031$p";

/// ModeInBandResize: reports terminal resize events as escape sequences. The
/// terminal sends `CSI 48 ; cellsHeight ; cellsWidth ; pixelHeight ;
/// pixelWidth t`. `2048`.
///
/// See: <https://gist.github.com/rockorager/e695fb2924d36b2bcf1fff4a3704bd83>
pub const MODE_IN_BAND_RESIZE: DECMode = DECMode(2048);
/// Set sequence for [MODE_IN_BAND_RESIZE].
pub const SET_MODE_IN_BAND_RESIZE: &str = "\x1b[?2048h";
/// Reset sequence for [MODE_IN_BAND_RESIZE].
pub const RESET_MODE_IN_BAND_RESIZE: &str = "\x1b[?2048l";
/// Request sequence for [MODE_IN_BAND_RESIZE].
pub const REQUEST_MODE_IN_BAND_RESIZE: &str = "\x1b[?2048$p";

/// Win32Input: determines whether input is processed by the Win32 console and
/// Conpty. `9001`.
///
/// See: <https://github.com/microsoft/terminal/blob/main/doc/specs/%234999%20-%20Improved%20keyboard%20handling%20in%20Conpty.md>
pub const MODE_WIN32_INPUT: DECMode = DECMode(9001);
/// Set sequence for [MODE_WIN32_INPUT].
pub const SET_MODE_WIN32_INPUT: &str = "\x1b[?9001h";
/// Reset sequence for [MODE_WIN32_INPUT].
pub const RESET_MODE_WIN32_INPUT: &str = "\x1b[?9001l";
/// Request sequence for [MODE_WIN32_INPUT].
pub const REQUEST_MODE_WIN32_INPUT: &str = "\x1b[?9001$p";

// ---------------------------------------------------------------------------
// Deprecated mode constants (ports `ansi/mode_deprecated.go`).
// Deprecated: use the `MODE_*` / `SET_MODE_*` / `RESET_MODE_*` /
// `REQUEST_MODE_*` constants above instead.
// ---------------------------------------------------------------------------

/// Keyboard Action Mode (KAM) controls locking of the keyboard. `2`.
pub const KEYBOARD_ACTION_MODE: ANSIMode = ANSIMode(2);
/// Set sequence for [KEYBOARD_ACTION_MODE].
pub const SET_KEYBOARD_ACTION_MODE: &str = "\x1b[2h";
/// Reset sequence for [KEYBOARD_ACTION_MODE].
pub const RESET_KEYBOARD_ACTION_MODE: &str = "\x1b[2l";
/// Request sequence for [KEYBOARD_ACTION_MODE].
pub const REQUEST_KEYBOARD_ACTION_MODE: &str = "\x1b[2$p";

/// Insert/Replace Mode (IRM) determines whether characters are inserted or
/// replaced. `4`.
pub const INSERT_REPLACE_MODE: ANSIMode = ANSIMode(4);
/// Set sequence for [INSERT_REPLACE_MODE].
pub const SET_INSERT_REPLACE_MODE: &str = "\x1b[4h";
/// Reset sequence for [INSERT_REPLACE_MODE].
pub const RESET_INSERT_REPLACE_MODE: &str = "\x1b[4l";
/// Request sequence for [INSERT_REPLACE_MODE].
pub const REQUEST_INSERT_REPLACE_MODE: &str = "\x1b[4$p";

/// BiDirectional Support Mode (BDSM) determines whether the terminal supports
/// bidirectional text. `8`.
pub const BI_DIRECTIONAL_SUPPORT_MODE: ANSIMode = ANSIMode(8);
/// Set sequence for [BI_DIRECTIONAL_SUPPORT_MODE].
pub const SET_BI_DIRECTIONAL_SUPPORT_MODE: &str = "\x1b[8h";
/// Reset sequence for [BI_DIRECTIONAL_SUPPORT_MODE].
pub const RESET_BI_DIRECTIONAL_SUPPORT_MODE: &str = "\x1b[8l";
/// Request sequence for [BI_DIRECTIONAL_SUPPORT_MODE].
pub const REQUEST_BI_DIRECTIONAL_SUPPORT_MODE: &str = "\x1b[8$p";

/// Send Receive Mode (SRM) or Local Echo Mode determines whether the terminal
/// echoes characters. `12`.
pub const SEND_RECEIVE_MODE: ANSIMode = ANSIMode(12);
/// Alias for [SEND_RECEIVE_MODE].
pub const LOCAL_ECHO_MODE: ANSIMode = SEND_RECEIVE_MODE;
/// Set sequence for [SEND_RECEIVE_MODE].
pub const SET_SEND_RECEIVE_MODE: &str = "\x1b[12h";
/// Reset sequence for [SEND_RECEIVE_MODE].
pub const RESET_SEND_RECEIVE_MODE: &str = "\x1b[12l";
/// Request sequence for [SEND_RECEIVE_MODE].
pub const REQUEST_SEND_RECEIVE_MODE: &str = "\x1b[12$p";
/// Set sequence for [LOCAL_ECHO_MODE].
pub const SET_LOCAL_ECHO_MODE: &str = "\x1b[12h";
/// Reset sequence for [LOCAL_ECHO_MODE].
pub const RESET_LOCAL_ECHO_MODE: &str = "\x1b[12l";
/// Request sequence for [LOCAL_ECHO_MODE].
pub const REQUEST_LOCAL_ECHO_MODE: &str = "\x1b[12$p";

/// Line Feed/New Line Mode (LNM) determines whether the terminal interprets
/// line feed as new line. `20`.
pub const LINE_FEED_NEW_LINE_MODE: ANSIMode = ANSIMode(20);
/// Set sequence for [LINE_FEED_NEW_LINE_MODE].
pub const SET_LINE_FEED_NEW_LINE_MODE: &str = "\x1b[20h";
/// Reset sequence for [LINE_FEED_NEW_LINE_MODE].
pub const RESET_LINE_FEED_NEW_LINE_MODE: &str = "\x1b[20l";
/// Request sequence for [LINE_FEED_NEW_LINE_MODE].
pub const REQUEST_LINE_FEED_NEW_LINE_MODE: &str = "\x1b[20$p";

/// Cursor Keys Mode (DECCKM) determines whether cursor keys send ANSI or
/// application sequences. `1`.
pub const CURSOR_KEYS_MODE: DECMode = DECMode(1);
/// Set sequence for [CURSOR_KEYS_MODE].
pub const SET_CURSOR_KEYS_MODE: &str = "\x1b[?1h";
/// Reset sequence for [CURSOR_KEYS_MODE].
pub const RESET_CURSOR_KEYS_MODE: &str = "\x1b[?1l";
/// Request sequence for [CURSOR_KEYS_MODE].
pub const REQUEST_CURSOR_KEYS_MODE: &str = "\x1b[?1$p";

/// Enable sequence for Cursor Keys mode.
pub const ENABLE_CURSOR_KEYS: &str = "\x1b[?1h";
/// Disable sequence for Cursor Keys mode.
pub const DISABLE_CURSOR_KEYS: &str = "\x1b[?1l";

/// Origin Mode (DECOM) determines whether the cursor moves to home or margin
/// position. `6`.
pub const ORIGIN_MODE: DECMode = DECMode(6);
/// Set sequence for [ORIGIN_MODE].
pub const SET_ORIGIN_MODE: &str = "\x1b[?6h";
/// Reset sequence for [ORIGIN_MODE].
pub const RESET_ORIGIN_MODE: &str = "\x1b[?6l";
/// Request sequence for [ORIGIN_MODE].
pub const REQUEST_ORIGIN_MODE: &str = "\x1b[?6$p";

/// Auto Wrap Mode (DECAWM) determines whether the cursor wraps to the next
/// line. `7`.
pub const AUTO_WRAP_MODE: DECMode = DECMode(7);
/// Set sequence for [AUTO_WRAP_MODE].
pub const SET_AUTO_WRAP_MODE: &str = "\x1b[?7h";
/// Reset sequence for [AUTO_WRAP_MODE].
pub const RESET_AUTO_WRAP_MODE: &str = "\x1b[?7l";
/// Request sequence for [AUTO_WRAP_MODE].
pub const REQUEST_AUTO_WRAP_MODE: &str = "\x1b[?7$p";

/// X10 Mouse Mode determines whether the mouse reports on button presses. `9`.
pub const X10_MOUSE_MODE: DECMode = DECMode(9);
/// Set sequence for [X10_MOUSE_MODE].
pub const SET_X10_MOUSE_MODE: &str = "\x1b[?9h";
/// Reset sequence for [X10_MOUSE_MODE].
pub const RESET_X10_MOUSE_MODE: &str = "\x1b[?9l";
/// Request sequence for [X10_MOUSE_MODE].
pub const REQUEST_X10_MOUSE_MODE: &str = "\x1b[?9$p";

/// Text Cursor Enable Mode (DECTCEM) shows/hides the cursor. `25`.
pub const TEXT_CURSOR_ENABLE_MODE: DECMode = DECMode(25);
/// Set sequence for [TEXT_CURSOR_ENABLE_MODE].
pub const SET_TEXT_CURSOR_ENABLE_MODE: &str = "\x1b[?25h";
/// Reset sequence for [TEXT_CURSOR_ENABLE_MODE].
pub const RESET_TEXT_CURSOR_ENABLE_MODE: &str = "\x1b[?25l";
/// Request sequence for [TEXT_CURSOR_ENABLE_MODE].
pub const REQUEST_TEXT_CURSOR_ENABLE_MODE: &str = "\x1b[?25$p";

/// Text Cursor Enable mode. `25`.
pub const CURSOR_ENABLE_MODE: DECMode = DECMode(25);
/// Request sequence for cursor visibility.
pub const REQUEST_CURSOR_VISIBILITY: &str = "\x1b[?25$p";

/// Numeric Keypad Mode (DECNKM) determines whether the keypad sends
/// application or numeric sequences. `66`.
pub const NUMERIC_KEYPAD_MODE: DECMode = DECMode(66);
/// Set sequence for [NUMERIC_KEYPAD_MODE].
pub const SET_NUMERIC_KEYPAD_MODE: &str = "\x1b[?66h";
/// Reset sequence for [NUMERIC_KEYPAD_MODE].
pub const RESET_NUMERIC_KEYPAD_MODE: &str = "\x1b[?66l";
/// Request sequence for [NUMERIC_KEYPAD_MODE].
pub const REQUEST_NUMERIC_KEYPAD_MODE: &str = "\x1b[?66$p";

/// Backarrow Key Mode (DECBKM) determines whether the backspace key sends
/// backspace or delete. `67`.
pub const BACKARROW_KEY_MODE: DECMode = DECMode(67);
/// Set sequence for [BACKARROW_KEY_MODE].
pub const SET_BACKARROW_KEY_MODE: &str = "\x1b[?67h";
/// Reset sequence for [BACKARROW_KEY_MODE].
pub const RESET_BACKARROW_KEY_MODE: &str = "\x1b[?67l";
/// Request sequence for [BACKARROW_KEY_MODE].
pub const REQUEST_BACKARROW_KEY_MODE: &str = "\x1b[?67$p";

/// Left Right Margin Mode (DECLRMM) determines whether left and right margins
/// can be set. `69`.
pub const LEFT_RIGHT_MARGIN_MODE: DECMode = DECMode(69);
/// Set sequence for [LEFT_RIGHT_MARGIN_MODE].
pub const SET_LEFT_RIGHT_MARGIN_MODE: &str = "\x1b[?69h";
/// Reset sequence for [LEFT_RIGHT_MARGIN_MODE].
pub const RESET_LEFT_RIGHT_MARGIN_MODE: &str = "\x1b[?69l";
/// Request sequence for [LEFT_RIGHT_MARGIN_MODE].
pub const REQUEST_LEFT_RIGHT_MARGIN_MODE: &str = "\x1b[?69$p";

/// Normal Mouse Mode determines whether the mouse reports on button presses
/// and releases. `1000`.
pub const NORMAL_MOUSE_MODE: DECMode = DECMode(1000);
/// Set sequence for [NORMAL_MOUSE_MODE].
pub const SET_NORMAL_MOUSE_MODE: &str = "\x1b[?1000h";
/// Reset sequence for [NORMAL_MOUSE_MODE].
pub const RESET_NORMAL_MOUSE_MODE: &str = "\x1b[?1000l";
/// Request sequence for [NORMAL_MOUSE_MODE].
pub const REQUEST_NORMAL_MOUSE_MODE: &str = "\x1b[?1000$p";

/// VT Mouse Tracking mode. `1000`.
pub const MOUSE_MODE: DECMode = DECMode(1000);
/// Enable sequence for VT Mouse Tracking.
pub const ENABLE_MOUSE: &str = "\x1b[?1000h";
/// Disable sequence for VT Mouse Tracking.
pub const DISABLE_MOUSE: &str = "\x1b[?1000l";
/// Request sequence for VT Mouse Tracking.
pub const REQUEST_MOUSE: &str = "\x1b[?1000$p";

/// Highlight Mouse Tracking determines whether the mouse reports on button
/// presses and highlighted cells. `1001`.
pub const HIGHLIGHT_MOUSE_MODE: DECMode = DECMode(1001);
/// Set sequence for [HIGHLIGHT_MOUSE_MODE].
pub const SET_HIGHLIGHT_MOUSE_MODE: &str = "\x1b[?1001h";
/// Reset sequence for [HIGHLIGHT_MOUSE_MODE].
pub const RESET_HIGHLIGHT_MOUSE_MODE: &str = "\x1b[?1001l";
/// Request sequence for [HIGHLIGHT_MOUSE_MODE].
pub const REQUEST_HIGHLIGHT_MOUSE_MODE: &str = "\x1b[?1001$p";

/// VT Hilite Mouse Tracking mode. `1001`.
pub const MOUSE_HILITE_MODE: DECMode = DECMode(1001);
/// Enable sequence for VT Hilite Mouse Tracking.
pub const ENABLE_MOUSE_HILITE: &str = "\x1b[?1001h";
/// Disable sequence for VT Hilite Mouse Tracking.
pub const DISABLE_MOUSE_HILITE: &str = "\x1b[?1001l";
/// Request sequence for VT Hilite Mouse Tracking.
pub const REQUEST_MOUSE_HILITE: &str = "\x1b[?1001$p";

/// Button Event Mouse Tracking reports button-motion events when a button is
/// pressed. `1002`.
pub const BUTTON_EVENT_MOUSE_MODE: DECMode = DECMode(1002);
/// Set sequence for [BUTTON_EVENT_MOUSE_MODE].
pub const SET_BUTTON_EVENT_MOUSE_MODE: &str = "\x1b[?1002h";
/// Reset sequence for [BUTTON_EVENT_MOUSE_MODE].
pub const RESET_BUTTON_EVENT_MOUSE_MODE: &str = "\x1b[?1002l";
/// Request sequence for [BUTTON_EVENT_MOUSE_MODE].
pub const REQUEST_BUTTON_EVENT_MOUSE_MODE: &str = "\x1b[?1002$p";

/// Cell Motion Mouse Tracking mode. `1002`.
pub const MOUSE_CELL_MOTION_MODE: DECMode = DECMode(1002);
/// Enable sequence for Cell Motion Mouse Tracking.
pub const ENABLE_MOUSE_CELL_MOTION: &str = "\x1b[?1002h";
/// Disable sequence for Cell Motion Mouse Tracking.
pub const DISABLE_MOUSE_CELL_MOTION: &str = "\x1b[?1002l";
/// Request sequence for Cell Motion Mouse Tracking.
pub const REQUEST_MOUSE_CELL_MOTION: &str = "\x1b[?1002$p";

/// Any Event Mouse Tracking reports all motion events. `1003`.
pub const ANY_EVENT_MOUSE_MODE: DECMode = DECMode(1003);
/// Set sequence for [ANY_EVENT_MOUSE_MODE].
pub const SET_ANY_EVENT_MOUSE_MODE: &str = "\x1b[?1003h";
/// Reset sequence for [ANY_EVENT_MOUSE_MODE].
pub const RESET_ANY_EVENT_MOUSE_MODE: &str = "\x1b[?1003l";
/// Request sequence for [ANY_EVENT_MOUSE_MODE].
pub const REQUEST_ANY_EVENT_MOUSE_MODE: &str = "\x1b[?1003$p";

/// All Mouse Tracking mode. `1003`.
pub const MOUSE_ALL_MOTION_MODE: DECMode = DECMode(1003);
/// Enable sequence for All Mouse Tracking.
pub const ENABLE_MOUSE_ALL_MOTION: &str = "\x1b[?1003h";
/// Disable sequence for All Mouse Tracking.
pub const DISABLE_MOUSE_ALL_MOTION: &str = "\x1b[?1003l";
/// Request sequence for All Mouse Tracking.
pub const REQUEST_MOUSE_ALL_MOTION: &str = "\x1b[?1003$p";

/// Focus Event Mode determines whether the terminal reports focus and blur
/// events. `1004`.
pub const FOCUS_EVENT_MODE: DECMode = DECMode(1004);
/// Set sequence for [FOCUS_EVENT_MODE].
pub const SET_FOCUS_EVENT_MODE: &str = "\x1b[?1004h";
/// Reset sequence for [FOCUS_EVENT_MODE].
pub const RESET_FOCUS_EVENT_MODE: &str = "\x1b[?1004l";
/// Request sequence for [FOCUS_EVENT_MODE].
pub const REQUEST_FOCUS_EVENT_MODE: &str = "\x1b[?1004$p";

/// Focus reporting mode. `1004`.
pub const REPORT_FOCUS_MODE: DECMode = DECMode(1004);
/// Enable sequence for focus reporting.
pub const ENABLE_REPORT_FOCUS: &str = "\x1b[?1004h";
/// Disable sequence for focus reporting.
pub const DISABLE_REPORT_FOCUS: &str = "\x1b[?1004l";
/// Request sequence for focus reporting.
pub const REQUEST_REPORT_FOCUS: &str = "\x1b[?1004$p";

/// UTF-8 Extended Mouse Mode changes the mouse tracking encoding to use UTF-8
/// parameters. `1005`.
pub const UTF8_EXT_MOUSE_MODE: DECMode = DECMode(1005);
/// Set sequence for [UTF8_EXT_MOUSE_MODE].
pub const SET_UTF8_EXT_MOUSE_MODE: &str = "\x1b[?1005h";
/// Reset sequence for [UTF8_EXT_MOUSE_MODE].
pub const RESET_UTF8_EXT_MOUSE_MODE: &str = "\x1b[?1005l";
/// Request sequence for [UTF8_EXT_MOUSE_MODE].
pub const REQUEST_UTF8_EXT_MOUSE_MODE: &str = "\x1b[?1005$p";

/// SGR Extended Mouse Mode changes the mouse tracking encoding to use SGR
/// parameters. `1006`.
pub const SGR_EXT_MOUSE_MODE: DECMode = DECMode(1006);
/// Set sequence for [SGR_EXT_MOUSE_MODE].
pub const SET_SGR_EXT_MOUSE_MODE: &str = "\x1b[?1006h";
/// Reset sequence for [SGR_EXT_MOUSE_MODE].
pub const RESET_SGR_EXT_MOUSE_MODE: &str = "\x1b[?1006l";
/// Request sequence for [SGR_EXT_MOUSE_MODE].
pub const REQUEST_SGR_EXT_MOUSE_MODE: &str = "\x1b[?1006$p";

/// Mouse SGR Extended mode. `1006`.
pub const MOUSE_SGR_EXT_MODE: DECMode = DECMode(1006);
/// Enable sequence for Mouse SGR Extended mode.
pub const ENABLE_MOUSE_SGR_EXT: &str = "\x1b[?1006h";
/// Disable sequence for Mouse SGR Extended mode.
pub const DISABLE_MOUSE_SGR_EXT: &str = "\x1b[?1006l";
/// Request sequence for Mouse SGR Extended mode.
pub const REQUEST_MOUSE_SGR_EXT: &str = "\x1b[?1006$p";

/// URXVT Extended Mouse Mode changes the mouse tracking encoding to use an
/// alternate encoding. `1015`.
pub const URXVT_EXT_MOUSE_MODE: DECMode = DECMode(1015);
/// Set sequence for [URXVT_EXT_MOUSE_MODE].
pub const SET_URXVT_EXT_MOUSE_MODE: &str = "\x1b[?1015h";
/// Reset sequence for [URXVT_EXT_MOUSE_MODE].
pub const RESET_URXVT_EXT_MOUSE_MODE: &str = "\x1b[?1015l";
/// Request sequence for [URXVT_EXT_MOUSE_MODE].
pub const REQUEST_URXVT_EXT_MOUSE_MODE: &str = "\x1b[?1015$p";

/// SGR Pixel Extended Mouse Mode changes the mouse tracking encoding to use
/// SGR parameters with pixel coordinates. `1016`.
pub const SGR_PIXEL_EXT_MOUSE_MODE: DECMode = DECMode(1016);
/// Set sequence for [SGR_PIXEL_EXT_MOUSE_MODE].
pub const SET_SGR_PIXEL_EXT_MOUSE_MODE: &str = "\x1b[?1016h";
/// Reset sequence for [SGR_PIXEL_EXT_MOUSE_MODE].
pub const RESET_SGR_PIXEL_EXT_MOUSE_MODE: &str = "\x1b[?1016l";
/// Request sequence for [SGR_PIXEL_EXT_MOUSE_MODE].
pub const REQUEST_SGR_PIXEL_EXT_MOUSE_MODE: &str = "\x1b[?1016$p";

/// Alternate Screen Mode determines whether the alternate screen buffer is
/// active. `1047`.
pub const ALT_SCREEN_MODE: DECMode = DECMode(1047);
/// Set sequence for [ALT_SCREEN_MODE].
pub const SET_ALT_SCREEN_MODE: &str = "\x1b[?1047h";
/// Reset sequence for [ALT_SCREEN_MODE].
pub const RESET_ALT_SCREEN_MODE: &str = "\x1b[?1047l";
/// Request sequence for [ALT_SCREEN_MODE].
pub const REQUEST_ALT_SCREEN_MODE: &str = "\x1b[?1047$p";

/// Save Cursor Mode saves the cursor position. `1048`.
pub const SAVE_CURSOR_MODE: DECMode = DECMode(1048);
/// Set sequence for [SAVE_CURSOR_MODE].
pub const SET_SAVE_CURSOR_MODE: &str = "\x1b[?1048h";
/// Reset sequence for [SAVE_CURSOR_MODE].
pub const RESET_SAVE_CURSOR_MODE: &str = "\x1b[?1048l";
/// Request sequence for [SAVE_CURSOR_MODE].
pub const REQUEST_SAVE_CURSOR_MODE: &str = "\x1b[?1048$p";

/// Alternate Screen Save Cursor Mode saves the cursor position and switches to
/// alternate screen. `1049`.
pub const ALT_SCREEN_SAVE_CURSOR_MODE: DECMode = DECMode(1049);
/// Set sequence for [ALT_SCREEN_SAVE_CURSOR_MODE].
pub const SET_ALT_SCREEN_SAVE_CURSOR_MODE: &str = "\x1b[?1049h";
/// Reset sequence for [ALT_SCREEN_SAVE_CURSOR_MODE].
pub const RESET_ALT_SCREEN_SAVE_CURSOR_MODE: &str = "\x1b[?1049l";
/// Request sequence for [ALT_SCREEN_SAVE_CURSOR_MODE].
pub const REQUEST_ALT_SCREEN_SAVE_CURSOR_MODE: &str = "\x1b[?1049$p";

/// Alternate Screen Buffer mode. `1049`.
pub const ALT_SCREEN_BUFFER_MODE: DECMode = DECMode(1049);
/// Set sequence for [ALT_SCREEN_BUFFER_MODE].
pub const SET_ALT_SCREEN_BUFFER_MODE: &str = "\x1b[?1049h";
/// Reset sequence for [ALT_SCREEN_BUFFER_MODE].
pub const RESET_ALT_SCREEN_BUFFER_MODE: &str = "\x1b[?1049l";
/// Request sequence for [ALT_SCREEN_BUFFER_MODE].
pub const REQUEST_ALT_SCREEN_BUFFER_MODE: &str = "\x1b[?1049$p";
/// Enable sequence for the alternate screen buffer.
pub const ENABLE_ALT_SCREEN_BUFFER: &str = "\x1b[?1049h";
/// Disable sequence for the alternate screen buffer.
pub const DISABLE_ALT_SCREEN_BUFFER: &str = "\x1b[?1049l";
/// Request sequence for the alternate screen buffer.
pub const REQUEST_ALT_SCREEN_BUFFER: &str = "\x1b[?1049$p";

/// Bracketed Paste Mode determines whether pasted text is bracketed with
/// escape sequences. `2004`.
pub const BRACKETED_PASTE_MODE: DECMode = DECMode(2004);
/// Set sequence for [BRACKETED_PASTE_MODE].
pub const SET_BRACKETED_PASTE_MODE: &str = "\x1b[?2004h";
/// Reset sequence for [BRACKETED_PASTE_MODE].
pub const RESET_BRACKETED_PASTE_MODE: &str = "\x1b[?2004l";
/// Request sequence for [BRACKETED_PASTE_MODE].
pub const REQUEST_BRACKETED_PASTE_MODE: &str = "\x1b[?2004$p";
/// Enable sequence for bracketed paste.
pub const ENABLE_BRACKETED_PASTE: &str = "\x1b[?2004h";
/// Disable sequence for bracketed paste.
pub const DISABLE_BRACKETED_PASTE: &str = "\x1b[?2004l";
/// Request sequence for bracketed paste.
pub const REQUEST_BRACKETED_PASTE: &str = "\x1b[?2004$p";

/// Synchronized Output Mode determines whether output is synchronized with the
/// terminal. `2026`.
pub const SYNCHRONIZED_OUTPUT_MODE: DECMode = DECMode(2026);
/// Set sequence for [SYNCHRONIZED_OUTPUT_MODE].
pub const SET_SYNCHRONIZED_OUTPUT_MODE: &str = "\x1b[?2026h";
/// Reset sequence for [SYNCHRONIZED_OUTPUT_MODE].
pub const RESET_SYNCHRONIZED_OUTPUT_MODE: &str = "\x1b[?2026l";
/// Request sequence for [SYNCHRONIZED_OUTPUT_MODE].
pub const REQUEST_SYNCHRONIZED_OUTPUT_MODE: &str = "\x1b[?2026$p";

/// Synchronized output mode. `2026`.
pub const SYNCD_OUTPUT_MODE: DECMode = DECMode(2026);
/// Enable sequence for synchronized output.
pub const ENABLE_SYNCD_OUTPUT: &str = "\x1b[?2026h";
/// Disable sequence for synchronized output.
pub const DISABLE_SYNCD_OUTPUT: &str = "\x1b[?2026l";
/// Request sequence for synchronized output.
pub const REQUEST_SYNCD_OUTPUT: &str = "\x1b[?2026$p";

/// Unicode Core Mode determines whether the terminal uses Unicode grapheme
/// clustering. `2027`.
pub const UNICODE_CORE_MODE: DECMode = DECMode(2027);
/// Set sequence for [UNICODE_CORE_MODE].
pub const SET_UNICODE_CORE_MODE: &str = "\x1b[?2027h";
/// Reset sequence for [UNICODE_CORE_MODE].
pub const RESET_UNICODE_CORE_MODE: &str = "\x1b[?2027l";
/// Request sequence for [UNICODE_CORE_MODE].
pub const REQUEST_UNICODE_CORE_MODE: &str = "\x1b[?2027$p";

/// Grapheme Clustering Mode determines whether the terminal looks for grapheme
/// clusters. `2027`.
pub const GRAPHEME_CLUSTERING_MODE: DECMode = DECMode(2027);
/// Set sequence for [GRAPHEME_CLUSTERING_MODE].
pub const SET_GRAPHEME_CLUSTERING_MODE: &str = "\x1b[?2027h";
/// Reset sequence for [GRAPHEME_CLUSTERING_MODE].
pub const RESET_GRAPHEME_CLUSTERING_MODE: &str = "\x1b[?2027l";
/// Request sequence for [GRAPHEME_CLUSTERING_MODE].
pub const REQUEST_GRAPHEME_CLUSTERING_MODE: &str = "\x1b[?2027$p";
/// Enable sequence for grapheme clustering.
pub const ENABLE_GRAPHEME_CLUSTERING: &str = "\x1b[?2027h";
/// Disable sequence for grapheme clustering.
pub const DISABLE_GRAPHEME_CLUSTERING: &str = "\x1b[?2027l";
/// Request sequence for grapheme clustering.
pub const REQUEST_GRAPHEME_CLUSTERING: &str = "\x1b[?2027$p";

/// Light Dark Mode enables reporting the operating system's color scheme
/// preference. `2031`.
pub const LIGHT_DARK_MODE: DECMode = DECMode(2031);
/// Set sequence for [LIGHT_DARK_MODE].
pub const SET_LIGHT_DARK_MODE: &str = "\x1b[?2031h";
/// Reset sequence for [LIGHT_DARK_MODE].
pub const RESET_LIGHT_DARK_MODE: &str = "\x1b[?2031l";
/// Request sequence for [LIGHT_DARK_MODE].
pub const REQUEST_LIGHT_DARK_MODE: &str = "\x1b[?2031$p";

/// In Band Resize Mode reports terminal resize events as escape sequences.
/// `2048`.
pub const IN_BAND_RESIZE_MODE: DECMode = DECMode(2048);
/// Set sequence for [IN_BAND_RESIZE_MODE].
pub const SET_IN_BAND_RESIZE_MODE: &str = "\x1b[?2048h";
/// Reset sequence for [IN_BAND_RESIZE_MODE].
pub const RESET_IN_BAND_RESIZE_MODE: &str = "\x1b[?2048l";
/// Request sequence for [IN_BAND_RESIZE_MODE].
pub const REQUEST_IN_BAND_RESIZE_MODE: &str = "\x1b[?2048$p";

/// Win32Input determines whether input is processed by the Win32 console and
/// Conpty. `9001`.
pub const WIN32_INPUT_MODE: DECMode = DECMode(9001);
/// Set sequence for [WIN32_INPUT_MODE].
pub const SET_WIN32_INPUT_MODE: &str = "\x1b[?9001h";
/// Reset sequence for [WIN32_INPUT_MODE].
pub const RESET_WIN32_INPUT_MODE: &str = "\x1b[?9001l";
/// Request sequence for [WIN32_INPUT_MODE].
pub const REQUEST_WIN32_INPUT_MODE: &str = "\x1b[?9001$p";
/// Enable sequence for Win32 input.
pub const ENABLE_WIN32_INPUT: &str = "\x1b[?9001h";
/// Disable sequence for Win32 input.
pub const DISABLE_WIN32_INPUT: &str = "\x1b[?9001l";
/// Request sequence for Win32 input.
pub const REQUEST_WIN32_INPUT: &str = "\x1b[?9001$p";

// ---------------------------------------------------------------------------
// Modes map (ports `ansi/modes.go`).
// ---------------------------------------------------------------------------

/// Modes represents the terminal modes that can be set or reset. By default,
/// all modes are [ModeSetting::NOT_RECOGNIZED].
#[derive(Debug, Clone, Default)]
pub struct Modes {
    modes: HashMap<Mode, ModeSetting>,
}

impl Modes {
    /// Creates a new empty modes map.
    pub fn new() -> Modes {
        Modes::default()
    }

    /// Get returns the setting of a terminal mode. If the mode is not set, it
    /// returns [ModeSetting::NOT_RECOGNIZED].
    pub fn get(&self, mode: Mode) -> ModeSetting {
        self.modes.get(&mode).copied().unwrap_or(ModeSetting::NOT_RECOGNIZED)
    }

    /// Delete deletes a terminal mode. This has the same effect as setting the
    /// mode to [ModeSetting::NOT_RECOGNIZED].
    pub fn delete(&mut self, mode: Mode) {
        self.modes.remove(&mode);
    }

    /// Set sets a terminal mode to [ModeSetting::SET].
    pub fn set(&mut self, modes: &[Mode]) {
        for &mode in modes {
            self.modes.insert(mode, ModeSetting::SET);
        }
    }

    /// PermanentlySet sets a terminal mode to [ModeSetting::PERMANENTLY_SET].
    pub fn permanently_set(&mut self, modes: &[Mode]) {
        for &mode in modes {
            self.modes.insert(mode, ModeSetting::PERMANENTLY_SET);
        }
    }

    /// Reset sets a terminal mode to [ModeSetting::RESET].
    pub fn reset(&mut self, modes: &[Mode]) {
        for &mode in modes {
            self.modes.insert(mode, ModeSetting::RESET);
        }
    }

    /// PermanentlyReset sets a terminal mode to [ModeSetting::PERMANENTLY_RESET].
    pub fn permanently_reset(&mut self, modes: &[Mode]) {
        for &mode in modes {
            self.modes.insert(mode, ModeSetting::PERMANENTLY_RESET);
        }
    }

    /// IsSet returns true if the mode is set to [ModeSetting::SET] or
    /// [ModeSetting::PERMANENTLY_SET].
    pub fn is_set(&self, mode: Mode) -> bool {
        self.get(mode).is_set()
    }

    /// IsPermanentlySet returns true if the mode is set to
    /// [ModeSetting::PERMANENTLY_SET].
    pub fn is_permanently_set(&self, mode: Mode) -> bool {
        self.get(mode).is_permanently_set()
    }

    /// IsReset returns true if the mode is set to [ModeSetting::RESET] or
    /// [ModeSetting::PERMANENTLY_RESET].
    pub fn is_reset(&self, mode: Mode) -> bool {
        self.get(mode).is_reset()
    }

    /// IsPermanentlyReset returns true if the mode is set to
    /// [ModeSetting::PERMANENTLY_RESET].
    pub fn is_permanently_reset(&self, mode: Mode) -> bool {
        self.get(mode).is_permanently_reset()
    }
}

/// ResetModifyOtherKeys disables modifyOtherKeys.
///
/// NOTE: upstream this lives in `ansi/xterm.go`; folded here.
pub const RESET_MODIFY_OTHER_KEYS: &str = "\x1b[>4m";

/// EnableModifyOtherKeys2 enables modifyOtherKeys mode 2.
pub const ENABLE_MODIFY_OTHER_KEYS2: &str = "\x1b[>4;2m";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_setting_methods() {
        let cases: &[(&str, ModeSetting, bool, bool, bool, bool, bool)] = &[
            ("ModeNotRecognized", ModeSetting(0), true, false, false, false, false),
            ("ModeSet", ModeSetting(1), false, true, false, false, false),
            ("ModeReset", ModeSetting(2), false, false, true, false, false),
            ("ModePermanentlySet", ModeSetting(3), false, true, false, true, false),
            ("ModePermanentlyReset", ModeSetting(4), false, false, true, false, true),
        ];
        for (name, mode, not_recog, is_set, is_reset, perm_set, perm_rst) in cases {
            assert_eq!(mode.is_not_recognized(), *not_recog, "{} IsNotRecognized", name);
            assert_eq!(mode.is_set(), *is_set, "{} IsSet", name);
            assert_eq!(mode.is_reset(), *is_reset, "{} IsReset", name);
            assert_eq!(mode.is_permanently_set(), *perm_set, "{} IsPermanentlySet", name);
            assert_eq!(mode.is_permanently_reset(), *perm_rst, "{} IsPermanentlyReset", name);
        }
        assert_eq!(ModeSetting::NOT_RECOGNIZED.0, 0);
        assert_eq!(ModeSetting::SET.0, 1);
        assert_eq!(ModeSetting::RESET.0, 2);
        assert_eq!(ModeSetting::PERMANENTLY_SET.0, 3);
        assert_eq!(ModeSetting::PERMANENTLY_RESET.0, 4);
    }

    #[test]
    fn test_set_mode() {
        let cases: &[(&str, &[Mode], &str)] = &[
            ("empty modes", &[], ""),
            ("single ANSI mode", &[Mode::from(MODE_KEYBOARD_ACTION)], "\x1b[2h"),
            ("single DEC mode", &[Mode::from(MODE_CURSOR_KEYS)], "\x1b[?1h"),
            (
                "multiple ANSI modes",
                &[Mode::from(MODE_KEYBOARD_ACTION), Mode::from(MODE_INSERT_REPLACE)],
                "\x1b[2;4h",
            ),
            (
                "multiple DEC modes",
                &[Mode::from(MODE_CURSOR_KEYS), Mode::from(MODE_AUTO_WRAP)],
                "\x1b[?1;7h",
            ),
            (
                "mixed ANSI and DEC modes",
                &[Mode::from(MODE_KEYBOARD_ACTION), Mode::from(MODE_CURSOR_KEYS)],
                "\x1b[2h\x1b[?1h",
            ),
            (
                "multiple mixed ANSI and DEC modes",
                &[
                    Mode::from(MODE_KEYBOARD_ACTION),
                    Mode::from(MODE_INSERT_REPLACE),
                    Mode::from(MODE_CURSOR_KEYS),
                    Mode::from(MODE_AUTO_WRAP),
                ],
                "\x1b[2;4h\x1b[?1;7h",
            ),
        ];
        for (name, modes, expected) in cases {
            assert_eq!(set_mode(modes), *expected, "{}", name);
        }
    }

    #[test]
    fn test_reset_mode() {
        let cases: &[(&str, &[Mode], &str)] = &[
            ("empty modes", &[], ""),
            ("single ANSI mode", &[Mode::from(MODE_KEYBOARD_ACTION)], "\x1b[2l"),
            ("single DEC mode", &[Mode::from(MODE_CURSOR_KEYS)], "\x1b[?1l"),
            (
                "multiple ANSI modes",
                &[Mode::from(MODE_KEYBOARD_ACTION), Mode::from(MODE_INSERT_REPLACE)],
                "\x1b[2;4l",
            ),
            (
                "multiple DEC modes",
                &[Mode::from(MODE_CURSOR_KEYS), Mode::from(MODE_AUTO_WRAP)],
                "\x1b[?1;7l",
            ),
            (
                "mixed ANSI and DEC modes",
                &[Mode::from(MODE_KEYBOARD_ACTION), Mode::from(MODE_CURSOR_KEYS)],
                "\x1b[2l\x1b[?1l",
            ),
            (
                "multiple mixed ANSI and DEC modes",
                &[
                    Mode::from(MODE_KEYBOARD_ACTION),
                    Mode::from(MODE_INSERT_REPLACE),
                    Mode::from(MODE_CURSOR_KEYS),
                    Mode::from(MODE_AUTO_WRAP),
                ],
                "\x1b[2;4l\x1b[?1;7l",
            ),
        ];
        for (name, modes, expected) in cases {
            assert_eq!(reset_mode(modes), *expected, "{}", name);
        }
    }

    #[test]
    fn test_request_mode() {
        assert_eq!(request_mode(Mode::from(MODE_KEYBOARD_ACTION)), "\x1b[2$p");
        assert_eq!(request_mode(Mode::from(MODE_CURSOR_KEYS)), "\x1b[?1$p");
    }

    #[test]
    fn test_report_mode() {
        assert_eq!(
            report_mode(Mode::from(MODE_KEYBOARD_ACTION), ModeSetting(0)),
            "\x1b[2;0$y"
        );
        assert_eq!(
            report_mode(Mode::from(MODE_CURSOR_KEYS), ModeSetting(1)),
            "\x1b[?1;1$y"
        );
        assert_eq!(
            report_mode(Mode::from(MODE_INSERT_REPLACE), ModeSetting(2)),
            "\x1b[4;2$y"
        );
        assert_eq!(
            report_mode(Mode::from(MODE_AUTO_WRAP), ModeSetting(3)),
            "\x1b[?7;3$y"
        );
        assert_eq!(
            report_mode(Mode::from(MODE_SEND_RECEIVE), ModeSetting(4)),
            "\x1b[12;4$y"
        );
        assert_eq!(
            report_mode(Mode::from(MODE_KEYBOARD_ACTION), ModeSetting(5)),
            "\x1b[2;0$y"
        );
    }

    #[test]
    fn test_mode_implementations() {
        assert_eq!(ANSIMode(42).mode(), 42);
        assert_eq!(DECMode(99).mode(), 99);
        assert_eq!(Mode::from(ANSIMode(42)).mode(), 42);
        assert_eq!(Mode::from(DECMode(99)).mode(), 99);
    }

    #[test]
    fn test_mode_alias_sequences() {
        assert_eq!(SET_MODE_MOUSE_X10, "\x1b[?9h");
        assert_eq!(RESET_MODE_MOUSE_X10, "\x1b[?9l");
        assert_eq!(SET_MODE_MOUSE_NORMAL, "\x1b[?1000h");
        assert_eq!(RESET_MODE_MOUSE_NORMAL, "\x1b[?1000l");
        assert_eq!(SET_MODE_MOUSE_BUTTON_EVENT, "\x1b[?1002h");
        assert_eq!(RESET_MODE_MOUSE_BUTTON_EVENT, "\x1b[?1002l");
        assert_eq!(SET_MODE_MOUSE_ANY_EVENT, "\x1b[?1003h");
        assert_eq!(RESET_MODE_MOUSE_ANY_EVENT, "\x1b[?1003l");
        assert_eq!(SET_MODE_BRACKETED_PASTE, "\x1b[?2004h");
        assert_eq!(RESET_MODE_BRACKETED_PASTE, "\x1b[?2004l");
        assert_eq!(SET_MODE_MOUSE_EXT_SGR, "\x1b[?1006h");
        assert_eq!(RESET_MODE_MOUSE_EXT_SGR, "\x1b[?1006l");
        assert_eq!(SET_MODE_MOUSE_EXT_URXVT, "\x1b[?1015h");
        assert_eq!(RESET_MODE_MOUSE_EXT_URXVT, "\x1b[?1015l");
        assert_eq!(SET_MODE_MOUSE_EXT_SGR_PIXEL, "\x1b[?1016h");
        assert_eq!(RESET_MODE_MOUSE_EXT_SGR_PIXEL, "\x1b[?1016l");
        assert_eq!(SET_MODE_UNICODE_CORE, "\x1b[?2027h");
        assert_eq!(SET_MODE_IN_BAND_RESIZE, "\x1b[?2048h");
        assert_eq!(SET_MODE_SYNCHRONIZED_OUTPUT, "\x1b[?2026h");
        assert_eq!(SHOW_CURSOR, "\x1b[?25h");
        assert_eq!(HIDE_CURSOR, "\x1b[?25l");
        assert_eq!(MODE_KEYBOARD_ACTION.0, 2);
        assert_eq!(MODE_MOUSE_X10.0, 9);
        assert_eq!(MODE_MOUSE_NORMAL.0, 1000);
        assert_eq!(MODE_MOUSE_BUTTON_EVENT.0, 1002);
        assert_eq!(MODE_MOUSE_ANY_EVENT.0, 1003);
        assert_eq!(MODE_BRACKETED_PASTE.0, 2004);
    }

    #[test]
    fn test_modes_map() {
        let mut modes = Modes::new();
        assert_eq!(modes.get(Mode::from(MODE_CURSOR_KEYS)), ModeSetting(0));
        modes.set(&[Mode::from(MODE_CURSOR_KEYS)]);
        assert!(modes.is_set(Mode::from(MODE_CURSOR_KEYS)));
        assert!(!modes.is_reset(Mode::from(MODE_CURSOR_KEYS)));
        modes.permanently_set(&[Mode::from(MODE_AUTO_WRAP)]);
        assert!(modes.is_permanently_set(Mode::from(MODE_AUTO_WRAP)));
        assert!(modes.is_set(Mode::from(MODE_AUTO_WRAP)));
        modes.reset(&[Mode::from(MODE_ORIGIN)]);
        assert!(modes.is_reset(Mode::from(MODE_ORIGIN)));
        modes.permanently_reset(&[Mode::from(MODE_ORIGIN)]);
        assert!(modes.is_permanently_reset(Mode::from(MODE_ORIGIN)));
        assert!(modes.is_reset(Mode::from(MODE_ORIGIN)));
        modes.delete(Mode::from(MODE_ORIGIN));
        assert_eq!(modes.get(Mode::from(MODE_ORIGIN)), ModeSetting(0));
    }
}
