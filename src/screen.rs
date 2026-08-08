//! Cleanroom Rust port of upstream Go source file: `ansi/screen.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! Screen control sequences: erase display/line, scroll up/down, insert and
//! delete lines and characters, scrolling-region margins, tab-stop control,
//! and report sequences.
//! </public-docs>

/// EraseDisplay (ED) clears the display or parts of the display. A screen is
/// the shown part of the terminal display excluding the scrollback buffer.
/// Possible values (default 0):
///
/// - `0`: clear from cursor to end of screen.
/// - `1`: clear from cursor to beginning of the screen.
/// - `2`: clear entire screen.
/// - `3`: clear entire display which deletes all lines saved in the
///   scrollback buffer (xterm).
///
/// `CSI <n> J`
pub fn erase_display(n: i32) -> String {
    let mut s = String::new();
    if n > 0 {
        s = n.to_string();
    }
    format!("\x1b[{s}J")
}

/// ED is an alias for [erase_display].
pub fn ed(n: i32) -> String {
    erase_display(n)
}

/// EraseDisplay constants.
/// These are the possible values for the EraseDisplay function.
/// Clear from cursor to end of screen.
pub const ERASE_SCREEN_BELOW: &str = "\x1b[J";
/// Clear from cursor to beginning of the screen.
pub const ERASE_SCREEN_ABOVE: &str = "\x1b[1J";
/// Clear entire screen.
pub const ERASE_ENTIRE_SCREEN: &str = "\x1b[2J";
/// Clear entire display which deletes all lines saved in the scrollback
/// buffer (xterm).
pub const ERASE_ENTIRE_DISPLAY: &str = "\x1b[3J";

/// EraseLine (EL) clears the current line or parts of the line. Possible
/// values:
///
/// - `0`: clear from cursor to end of line.
/// - `1`: clear from cursor to beginning of the line.
/// - `2`: clear entire line.
///
/// The cursor position is not affected.
///
/// `CSI <n> K`
pub fn erase_line(n: i32) -> String {
    let mut s = String::new();
    if n > 0 {
        s = n.to_string();
    }
    format!("\x1b[{s}K")
}

/// EL is an alias for [erase_line].
pub fn el(n: i32) -> String {
    erase_line(n)
}

/// EraseLine constants.
/// These are the possible values for the EraseLine function.
/// Clear from cursor to end of line.
pub const ERASE_LINE_RIGHT: &str = "\x1b[K";
/// Clear from cursor to beginning of the line.
pub const ERASE_LINE_LEFT: &str = "\x1b[1K";
/// Clear entire line.
pub const ERASE_ENTIRE_LINE: &str = "\x1b[2K";

/// ScrollUp (SU) scrolls the screen up n lines. New lines are added at the
/// bottom of the screen.
///
/// `CSI Pn S`
pub fn scroll_up(n: i32) -> String {
    let mut s = String::new();
    if n > 1 {
        s = n.to_string();
    }
    format!("\x1b[{s}S")
}

/// PanDown is an alias for [scroll_up].
pub fn pan_down(n: i32) -> String {
    scroll_up(n)
}

/// SU is an alias for [scroll_up].
pub fn su(n: i32) -> String {
    scroll_up(n)
}

/// ScrollDown (SD) scrolls the screen down n lines. New lines are added at
/// the top of the screen.
///
/// `CSI Pn T`
pub fn scroll_down(n: i32) -> String {
    let mut s = String::new();
    if n > 1 {
        s = n.to_string();
    }
    format!("\x1b[{s}T")
}

/// PanUp is an alias for [scroll_down].
pub fn pan_up(n: i32) -> String {
    scroll_down(n)
}

/// SD is an alias for [scroll_down].
pub fn sd(n: i32) -> String {
    scroll_down(n)
}

/// InsertLine (IL) inserts n blank lines at the current cursor position.
/// Existing lines are moved down.
///
/// `CSI Pn L`
pub fn insert_line(n: i32) -> String {
    let mut s = String::new();
    if n > 1 {
        s = n.to_string();
    }
    format!("\x1b[{s}L")
}

/// IL is an alias for [insert_line].
pub fn il(n: i32) -> String {
    insert_line(n)
}

/// DeleteLine (DL) deletes n lines at the current cursor position. Existing
/// lines are moved up.
///
/// `CSI Pn M`
pub fn delete_line(n: i32) -> String {
    let mut s = String::new();
    if n > 1 {
        s = n.to_string();
    }
    format!("\x1b[{s}M")
}

/// DL is an alias for [delete_line].
pub fn dl(n: i32) -> String {
    delete_line(n)
}

/// SetTopBottomMargins (DECSTBM) sets the top and bottom margins for the
/// scrolling region. The default is the entire screen.
///
/// Default is 1 and the bottom of the screen.
///
/// `CSI Pt ; Pb r`
pub fn set_top_bottom_margins(top: i32, bot: i32) -> String {
    let mut t = String::new();
    if top > 0 {
        t = top.to_string();
    }
    let mut b = String::new();
    if bot > 0 {
        b = bot.to_string();
    }
    format!("\x1b[{t};{b}r")
}

/// DECSTBM is an alias for [set_top_bottom_margins].
pub fn decstbm(top: i32, bot: i32) -> String {
    set_top_bottom_margins(top, bot)
}

/// SetLeftRightMargins (DECSLRM) sets the left and right margins for the
/// scrolling region.
///
/// Default is 1 and the right of the screen.
///
/// `CSI Pl ; Pr s`
pub fn set_left_right_margins(left: i32, right: i32) -> String {
    let mut l = String::new();
    if left > 0 {
        l = left.to_string();
    }
    let mut r = String::new();
    if right > 0 {
        r = right.to_string();
    }
    format!("\x1b[{l};{r}s")
}

/// DECSLRM is an alias for [set_left_right_margins].
pub fn decslrm(left: i32, right: i32) -> String {
    set_left_right_margins(left, right)
}

/// SetScrollingRegion (DECSTBM) sets the top and bottom margins for the
/// scrolling region. The default is the entire screen.
///
/// `CSI <top> ; <bottom> r`
///
/// Deprecated: use [set_top_bottom_margins] instead.
pub fn set_scrolling_region(t: i32, b: i32) -> String {
    let t = if t < 0 { 0 } else { t };
    let b = if b < 0 { 0 } else { b };
    format!("\x1b[{t};{b}r")
}

/// InsertCharacter (ICH) inserts n blank characters at the current cursor
/// position. Existing characters move to the right. Characters moved past the
/// right margin are lost. ICH has no effect outside the scrolling margins.
///
/// Default is 1.
///
/// `CSI Pn @`
pub fn insert_character(n: i32) -> String {
    let mut s = String::new();
    if n > 1 {
        s = n.to_string();
    }
    format!("\x1b[{s}@")
}

/// ICH is an alias for [insert_character].
pub fn ich(n: i32) -> String {
    insert_character(n)
}

/// DeleteCharacter (DCH) deletes n characters at the current cursor position.
/// As the characters are deleted, the remaining characters move to the left
/// and the cursor remains at the same position.
///
/// Default is 1.
///
/// `CSI Pn P`
pub fn delete_character(n: i32) -> String {
    let mut s = String::new();
    if n > 1 {
        s = n.to_string();
    }
    format!("\x1b[{s}P")
}

/// DCH is an alias for [delete_character].
pub fn dch(n: i32) -> String {
    delete_character(n)
}

/// SetTabEvery8Columns (DECST8C) sets the tab stops at every 8 columns.
///
/// `CSI ? 5 W`
pub const SET_TAB_EVERY_8_COLUMNS: &str = "\x1b[?5W";
/// DECST8C is an alias for [SET_TAB_EVERY_8_COLUMNS].
pub const DECST8C: &str = SET_TAB_EVERY_8_COLUMNS;

/// HorizontalTabSet (HTS) sets a horizontal tab stop at the current cursor
/// column.
///
/// `ESC H`
pub const HORIZONTAL_TAB_SET: &str = "\x1bH";

/// TabClear (TBC) clears tab stops.
///
/// Default is 0.
///
/// Possible values:
/// - `0`: clear tab stop at the current column. (default)
/// - `3`: clear all tab stops.
///
/// `CSI Pn g`
pub fn tab_clear(n: i32) -> String {
    let mut s = String::new();
    if n > 0 {
        s = n.to_string();
    }
    format!("\x1b[{s}g")
}

/// TBC is an alias for [tab_clear].
pub fn tbc(n: i32) -> String {
    tab_clear(n)
}

/// RequestPresentationStateReport (DECRQPSR) requests a presentation state
/// report.
///
/// `CSI Pn $ p`
pub fn request_presentation_state_report(n: i32) -> String {
    format!("\x1b[{n}$p")
}

/// DECRQPSR is an alias for [request_presentation_state_report].
pub fn decrqpsr(n: i32) -> String {
    request_presentation_state_report(n)
}

/// TabStopReport (DECTABSR) reports the current tab stops.
///
/// `CSI 0 ; <stops> u`
pub fn tab_stop_report(stops: &[i32]) -> String {
    let mut s = String::new();
    for stop in stops {
        s.push_str(&format!("{stop};"));
    }
    format!("\x1b[0;{s}u")
}

/// DECTABSR is an alias for [tab_stop_report].
pub fn dectabsr(stops: &[i32]) -> String {
    tab_stop_report(stops)
}

/// CursorInformationReport (DECCIR) reports the current cursor position.
///
/// `CSI <values> R`
pub fn cursor_information_report(values: &[i32]) -> String {
    let mut s = String::new();
    for v in values {
        s.push_str(&format!("{v};"));
    }
    format!("\x1b[{s}R")
}

/// DECCIR is an alias for [cursor_information_report].
pub fn deccir(values: &[i32]) -> String {
    cursor_information_report(values)
}

/// RepeatPreviousCharacter (REP) repeats the previous character n times.
///
/// `CSI Pn b`
pub fn repeat_previous_character(n: i32) -> String {
    let mut s = String::new();
    if n > 1 {
        s = n.to_string();
    }
    format!("\x1b[{s}b")
}

/// REP is an alias for [repeat_previous_character].
pub fn rep(n: i32) -> String {
    repeat_previous_character(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erase_display() {
        assert_eq!(erase_display(0), "\x1b[J");
        assert_eq!(erase_display(2), "\x1b[2J");
        assert_eq!(ERASE_SCREEN_BELOW, "\x1b[J");
        assert_eq!(ERASE_ENTIRE_SCREEN, "\x1b[2J");
    }

    #[test]
    fn test_erase_line() {
        assert_eq!(erase_line(0), "\x1b[K");
        assert_eq!(erase_line(1), "\x1b[1K");
        assert_eq!(ERASE_LINE_RIGHT, "\x1b[K");
        assert_eq!(ERASE_LINE_LEFT, "\x1b[1K");
    }

    #[test]
    fn test_scroll() {
        assert_eq!(scroll_up(1), "\x1b[S");
        assert_eq!(scroll_up(3), "\x1b[3S");
        assert_eq!(scroll_down(1), "\x1b[T");
        assert_eq!(scroll_down(4), "\x1b[4T");
    }

    #[test]
    fn test_lines() {
        assert_eq!(insert_line(1), "\x1b[L");
        assert_eq!(insert_line(2), "\x1b[2L");
        assert_eq!(delete_line(1), "\x1b[M");
        assert_eq!(delete_line(5), "\x1b[5M");
    }

    #[test]
    fn test_margins() {
        assert_eq!(set_top_bottom_margins(0, 0), "\x1b[;r");
        assert_eq!(set_top_bottom_margins(3, 10), "\x1b[3;10r");
    }

    #[test]
    fn test_characters() {
        assert_eq!(insert_character(1), "\x1b[@");
        assert_eq!(insert_character(8), "\x1b[8@");
        assert_eq!(delete_character(1), "\x1b[P");
        assert_eq!(delete_character(3), "\x1b[3P");
        assert_eq!(repeat_previous_character(1), "\x1b[b");
        assert_eq!(repeat_previous_character(9), "\x1b[9b");
    }

    #[test]
    fn test_tabs() {
        assert_eq!(tab_clear(0), "\x1b[g");
        assert_eq!(tab_clear(3), "\x1b[3g");
        assert_eq!(SET_TAB_EVERY_8_COLUMNS, "\x1b[?5W");
        assert_eq!(HORIZONTAL_TAB_SET, "\x1bH");
    }

    #[test]
    fn test_reports() {
        assert_eq!(tab_stop_report(&[9, 17]), "\x1b[0;9;17;u");
        assert_eq!(cursor_information_report(&[2, 5]), "\x1b[2;5;R");
    }
}
