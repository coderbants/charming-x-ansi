//! Cleanroom Rust port of upstream Go source file: `ansi/cursor.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! Cursor movement, position, save/restore, and style sequences.
//! </public-docs>

/// SaveCursor (DECSC) is an escape sequence that saves the current cursor
/// position.
///
/// `ESC 7`
///
/// See: <https://vt100.net/docs/vt510-rm/DECSC.html>
pub const SAVE_CURSOR: &str = "\x1b7";
/// Alias for [SAVE_CURSOR].
pub const DECSC: &str = SAVE_CURSOR;

/// RestoreCursor (DECRC) is an escape sequence that restores the cursor
/// position.
///
/// `ESC 8`
///
/// See: <https://vt100.net/docs/vt510-rm/DECRC.html>
pub const RESTORE_CURSOR: &str = "\x1b8";
/// Alias for [RESTORE_CURSOR].
pub const DECRC: &str = RESTORE_CURSOR;

/// RequestCursorPosition is an escape sequence that requests the current
/// cursor position.
///
/// `CSI 6 n`
///
/// The terminal will report the cursor position as `CSI Pl ; Pc R`, where Pl
/// is the line number and Pc is the column number.
/// See: <https://vt100.net/docs/vt510-rm/CPR.html>
///
/// Deprecated: use [REQUEST_CURSOR_POSITION_REPORT] instead.
pub const REQUEST_CURSOR_POSITION: &str = "\x1b[6n";

/// RequestExtendedCursorPosition (DECXCPR) is a sequence for requesting the
/// cursor position report including the current page number.
///
/// `CSI ? 6 n`
///
/// Deprecated: use [REQUEST_EXTENDED_CURSOR_POSITION_REPORT] instead.
pub const REQUEST_EXTENDED_CURSOR_POSITION: &str = "\x1b[?6n";

/// CursorUp (CUU) returns a sequence for moving the cursor up n cells.
///
/// `CSI n A`
///
/// See: <https://vt100.net/docs/vt510-rm/CUU.html>
pub fn cursor_up(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}A", s)
}

/// CUU is an alias for [cursor_up].
pub fn cuu(n: i32) -> String {
    cursor_up(n)
}

/// CUU1 is a sequence for moving the cursor up one cell.
pub const CUU1: &str = "\x1b[A";

/// CursorUp1 is a sequence for moving the cursor up one cell.
/// This is equivalent to [cursor_up](1).
///
/// Deprecated: use [CUU1] instead.
pub const CURSOR_UP1: &str = "\x1b[A";

/// CursorDown (CUD) returns a sequence for moving the cursor down n cells.
///
/// `CSI n B`
///
/// See: <https://vt100.net/docs/vt510-rm/CUD.html>
pub fn cursor_down(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}B", s)
}

/// CUD is an alias for [cursor_down].
pub fn cud(n: i32) -> String {
    cursor_down(n)
}

/// CUD1 is a sequence for moving the cursor down one cell.
pub const CUD1: &str = "\x1b[B";

/// CursorDown1 is a sequence for moving the cursor down one cell.
/// This is equivalent to [cursor_down](1).
///
/// Deprecated: use [CUD1] instead.
pub const CURSOR_DOWN1: &str = "\x1b[B";

/// CursorForward (CUF) returns a sequence for moving the cursor right n cells.
///
/// `CSI n C`
///
/// See: <https://vt100.net/docs/vt510-rm/CUF.html>
pub fn cursor_forward(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}C", s)
}

/// CUF is an alias for [cursor_forward].
pub fn cuf(n: i32) -> String {
    cursor_forward(n)
}

/// CUF1 is a sequence for moving the cursor right one cell.
pub const CUF1: &str = "\x1b[C";

/// CursorRight (CUF) returns a sequence for moving the cursor right n cells.
///
/// `CSI n C`
///
/// See: <https://vt100.net/docs/vt510-rm/CUF.html>
///
/// Deprecated: use [cursor_forward] instead.
pub fn cursor_right(n: i32) -> String {
    cursor_forward(n)
}

/// CursorRight1 is a sequence for moving the cursor right one cell.
/// This is equivalent to [cursor_right](1).
///
/// Deprecated: use [CUF1] instead.
pub const CURSOR_RIGHT1: &str = CUF1;

/// CursorBackward (CUB) returns a sequence for moving the cursor left n cells.
///
/// `CSI n D`
///
/// See: <https://vt100.net/docs/vt510-rm/CUB.html>
pub fn cursor_backward(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}D", s)
}

/// CUB is an alias for [cursor_backward].
pub fn cub(n: i32) -> String {
    cursor_backward(n)
}

/// CUB1 is a sequence for moving the cursor left one cell.
pub const CUB1: &str = "\x1b[D";

/// CursorLeft (CUB) returns a sequence for moving the cursor left n cells.
///
/// `CSI n D`
///
/// See: <https://vt100.net/docs/vt510-rm/CUB.html>
///
/// Deprecated: use [cursor_backward] instead.
pub fn cursor_left(n: i32) -> String {
    cursor_backward(n)
}

/// CursorLeft1 is a sequence for moving the cursor left one cell.
/// This is equivalent to [cursor_left](1).
///
/// Deprecated: use [CUB1] instead.
pub const CURSOR_LEFT1: &str = CUB1;

/// CursorNextLine (CNL) returns a sequence for moving the cursor to the
/// beginning of the next line n times.
///
/// `CSI n E`
///
/// See: <https://vt100.net/docs/vt510-rm/CNL.html>
pub fn cursor_next_line(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}E", s)
}

/// CNL is an alias for [cursor_next_line].
pub fn cnl(n: i32) -> String {
    cursor_next_line(n)
}

/// CursorPreviousLine (CPL) returns a sequence for moving the cursor to the
/// beginning of the previous line n times.
///
/// `CSI n F`
///
/// See: <https://vt100.net/docs/vt510-rm/CPL.html>
pub fn cursor_previous_line(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}F", s)
}

/// CPL is an alias for [cursor_previous_line].
pub fn cpl(n: i32) -> String {
    cursor_previous_line(n)
}

/// CursorHorizontalAbsolute (CHA) returns a sequence for moving the cursor to
/// the given column.
///
/// Default is 1.
///
/// `CSI n G`
///
/// See: <https://vt100.net/docs/vt510-rm/CHA.html>
pub fn cursor_horizontal_absolute(col: i32) -> String {
    let s = if col > 0 { col.to_string() } else { String::new() };
    format!("\x1b[{}G", s)
}

/// CHA is an alias for [cursor_horizontal_absolute].
pub fn cha(col: i32) -> String {
    cursor_horizontal_absolute(col)
}

/// CursorPosition (CUP) returns a sequence for setting the cursor to the
/// given row and column.
///
/// Default is 1,1.
///
/// `CSI n ; m H`
///
/// See: <https://vt100.net/docs/vt510-rm/CUP.html>
pub fn cursor_position(col: i32, row: i32) -> String {
    if row <= 1 && col <= 1 {
        return CURSOR_HOME_POSITION.to_string();
    }

    let r = if row > 0 { row.to_string() } else { String::new() };
    let c = if col > 0 { col.to_string() } else { String::new() };
    format!("\x1b[{};{}H", r, c)
}

/// CUP is an alias for [cursor_position].
pub fn cup(col: i32, row: i32) -> String {
    cursor_position(col, row)
}

/// CursorHomePosition is a sequence for moving the cursor to the upper left
/// corner of the scrolling region.
///
/// This is equivalent to [cursor_position](1, 1).
pub const CURSOR_HOME_POSITION: &str = "\x1b[H";

/// SetCursorPosition (CUP) returns a sequence for setting the cursor to the
/// given row and column.
///
/// `CSI n ; m H`
///
/// See: <https://vt100.net/docs/vt510-rm/CUP.html>
///
/// Deprecated: use [cursor_position] instead.
pub fn set_cursor_position(col: i32, row: i32) -> String {
    if row <= 0 && col <= 0 {
        return HOME_CURSOR_POSITION.to_string();
    }

    let r = if row > 0 { row.to_string() } else { String::new() };
    let c = if col > 0 { col.to_string() } else { String::new() };
    format!("\x1b[{};{}H", r, c)
}

/// HomeCursorPosition is a sequence for moving the cursor to the upper left
/// corner of the scrolling region. This is equivalent to
/// `set_cursor_position(1, 1)`.
///
/// Deprecated: use [CURSOR_HOME_POSITION] instead.
pub const HOME_CURSOR_POSITION: &str = CURSOR_HOME_POSITION;

/// MoveCursor (CUP) returns a sequence for setting the cursor to the
/// given row and column.
///
/// `CSI n ; m H`
///
/// See: <https://vt100.net/docs/vt510-rm/CUP.html>
///
/// Deprecated: use [cursor_position] instead.
pub fn move_cursor(col: i32, row: i32) -> String {
    set_cursor_position(col, row)
}

/// CursorOrigin is a sequence for moving the cursor to the upper left corner
/// of the display. This is equivalent to `set_cursor_position(1, 1)`.
///
/// Deprecated: use [CURSOR_HOME_POSITION] instead.
pub const CURSOR_ORIGIN: &str = "\x1b[1;1H";

/// MoveCursorOrigin is a sequence for moving the cursor to the upper left
/// corner of the display. This is equivalent to `set_cursor_position(1, 1)`.
///
/// Deprecated: use [CURSOR_HOME_POSITION] instead.
pub const MOVE_CURSOR_ORIGIN: &str = CURSOR_ORIGIN;

/// CursorHorizontalForwardTab (CHT) returns a sequence for moving the cursor
/// to the next tab stop n times.
///
/// Default is 1.
///
/// `CSI n I`
///
/// See: <https://vt100.net/docs/vt510-rm/CHT.html>
pub fn cursor_horizontal_forward_tab(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}I", s)
}

/// CHT is an alias for [cursor_horizontal_forward_tab].
pub fn cht(n: i32) -> String {
    cursor_horizontal_forward_tab(n)
}

/// EraseCharacter (ECH) returns a sequence for erasing n characters from the
/// screen. This doesn't affect other cell attributes.
///
/// Default is 1.
///
/// `CSI n X`
///
/// See: <https://vt100.net/docs/vt510-rm/ECH.html>
pub fn erase_character(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}X", s)
}

/// ECH is an alias for [erase_character].
pub fn ech(n: i32) -> String {
    erase_character(n)
}

/// CursorBackwardTab (CBT) returns a sequence for moving the cursor to the
/// previous tab stop n times.
///
/// Default is 1.
///
/// `CSI n Z`
///
/// See: <https://vt100.net/docs/vt510-rm/CBT.html>
pub fn cursor_backward_tab(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}Z", s)
}

/// CBT is an alias for [cursor_backward_tab].
pub fn cbt(n: i32) -> String {
    cursor_backward_tab(n)
}

/// VerticalPositionAbsolute (VPA) returns a sequence for moving the cursor to
/// the given row.
///
/// Default is 1.
///
/// `CSI n d`
///
/// See: <https://vt100.net/docs/vt510-rm/VPA.html>
pub fn vertical_position_absolute(row: i32) -> String {
    let s = if row > 0 { row.to_string() } else { String::new() };
    format!("\x1b[{}d", s)
}

/// VPA is an alias for [vertical_position_absolute].
pub fn vpa(row: i32) -> String {
    vertical_position_absolute(row)
}

/// VerticalPositionRelative (VPR) returns a sequence for moving the cursor
/// down n rows relative to the current position.
///
/// Default is 1.
///
/// `CSI n e`
///
/// See: <https://vt100.net/docs/vt510-rm/VPR.html>
pub fn vertical_position_relative(n: i32) -> String {
    let s = if n > 1 { n.to_string() } else { String::new() };
    format!("\x1b[{}e", s)
}

/// VPR is an alias for [vertical_position_relative].
pub fn vpr(n: i32) -> String {
    vertical_position_relative(n)
}

/// HorizontalVerticalPosition (HVP) returns a sequence for moving the cursor
/// to the given row and column.
///
/// Default is 1,1.
///
/// `CSI n ; m f`
///
/// This has the same effect as [cursor_position].
///
/// See: <https://vt100.net/docs/vt510-rm/HVP.html>
pub fn horizontal_vertical_position(col: i32, row: i32) -> String {
    let r = if row > 0 { row.to_string() } else { String::new() };
    let c = if col > 0 { col.to_string() } else { String::new() };
    format!("\x1b[{};{}f", r, c)
}

/// HVP is an alias for [horizontal_vertical_position].
pub fn hvp(col: i32, row: i32) -> String {
    horizontal_vertical_position(col, row)
}

/// HorizontalVerticalHomePosition is a sequence for moving the cursor to the
/// upper left corner of the scrolling region. This is equivalent to
/// `horizontal_vertical_position(1, 1)`.
pub const HORIZONTAL_VERTICAL_HOME_POSITION: &str = "\x1b[f";

/// SaveCurrentCursorPosition (SCOSC) is a sequence for saving the current
/// cursor position for SCO console mode.
///
/// `CSI s`
///
/// This acts like [SAVE_CURSOR], except the page number where the cursor is
/// located is not saved.
///
/// See: <https://vt100.net/docs/vt510-rm/SCOSC.html>
pub const SAVE_CURRENT_CURSOR_POSITION: &str = "\x1b[s";
/// Alias for [SAVE_CURRENT_CURSOR_POSITION].
pub const SCOSC: &str = SAVE_CURRENT_CURSOR_POSITION;

/// SaveCursorPosition (SCP or SCOSC) is a sequence for saving the cursor
/// position.
///
/// `CSI s`
///
/// Deprecated: use [SAVE_CURRENT_CURSOR_POSITION] instead.
pub const SAVE_CURSOR_POSITION: &str = "\x1b[s";

/// RestoreCurrentCursorPosition (SCORC) is a sequence for restoring the
/// current cursor position for SCO console mode.
///
/// `CSI u`
///
/// This acts like [RESTORE_CURSOR], except the page number where the cursor
/// was saved is not restored.
///
/// See: <https://vt100.net/docs/vt510-rm/SCORC.html>
pub const RESTORE_CURRENT_CURSOR_POSITION: &str = "\x1b[u";
/// Alias for [RESTORE_CURRENT_CURSOR_POSITION].
pub const SCORC: &str = RESTORE_CURRENT_CURSOR_POSITION;

/// RestoreCursorPosition (RCP or SCORC) is a sequence for restoring the
/// cursor position.
///
/// `CSI u`
///
/// Deprecated: use [RESTORE_CURRENT_CURSOR_POSITION] instead.
pub const RESTORE_CURSOR_POSITION: &str = "\x1b[u";

/// SetCursorStyle (DECSCUSR) returns a sequence for changing the cursor style.
///
/// Default is 1.
///
/// `CSI Ps SP q`
///
/// Where Ps is the cursor style:
///
/// 0: Blinking block, 1: Blinking block (default), 2: Steady block,
/// 3: Blinking underline, 4: Steady underline, 5: Blinking bar (xterm),
/// 6: Steady bar (xterm).
///
/// See: <https://vt100.net/docs/vt510-rm/DECSCUSR.html>
pub fn set_cursor_style(style: i32) -> String {
    let style = if style < 0 { 0 } else { style };
    format!("\x1b[{} q", style)
}

/// DECSCUSR is an alias for [set_cursor_style].
pub fn decscusr(style: i32) -> String {
    set_cursor_style(style)
}

/// SetPointerShape returns a sequence for changing the mouse pointer cursor
/// shape. Use "default" for the default pointer shape.
///
/// `OSC 22 ; Pt ST`
/// `OSC 22 ; Pt BEL`
///
/// Where Pt is the pointer shape name. The name can be anything that the
/// operating system can understand. Some common names are: copy, crosshair,
/// default, ew-resize, n-resize, text, wait.
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Operating-System-Commands>
pub fn set_pointer_shape(shape: &str) -> String {
    format!("\x1b]22;{}\x07", shape)
}

/// ReverseIndex (RI) is an escape sequence for moving the cursor up one line
/// in the same column. If the cursor is at the top margin, the screen scrolls
/// down.
pub const REVERSE_INDEX: &str = "\x1bM";

/// HorizontalPositionAbsolute (HPA) returns a sequence for moving the cursor
/// to the given column. This has the same effect as [cursor_position].
///
/// Default is 1.
///
/// `CSI n \``
///
/// See: <https://vt100.net/docs/vt510-rm/HPA.html>
pub fn horizontal_position_absolute(col: i32) -> String {
    let s = if col > 0 { col.to_string() } else { String::new() };
    format!("\x1b[{}`", s)
}

/// HPA is an alias for [horizontal_position_absolute].
pub fn hpa(col: i32) -> String {
    horizontal_position_absolute(col)
}

/// HorizontalPositionRelative (HPR) returns a sequence for moving the cursor
/// right n columns relative to the current position. This has the same effect
/// as [cursor_position].
///
/// Default is 1.
///
/// `CSI n a`
///
/// See: <https://vt100.net/docs/vt510-rm/HPR.html>
pub fn horizontal_position_relative(n: i32) -> String {
    let s = if n > 0 { n.to_string() } else { String::new() };
    format!("\x1b[{}a", s)
}

/// HPR is an alias for [horizontal_position_relative].
pub fn hpr(n: i32) -> String {
    horizontal_position_relative(n)
}

/// Index (IND) is an escape sequence for moving the cursor down one line in
/// the same column. If the cursor is at the bottom margin, the screen scrolls
/// up.
pub const INDEX: &str = "\x1bD";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_restore_cursor() {
        assert_eq!(SAVE_CURSOR, "\x1b7");
        assert_eq!(DECSC, "\x1b7");
        assert_eq!(RESTORE_CURSOR, "\x1b8");
        assert_eq!(DECRC, "\x1b8");
        assert_eq!(SAVE_CURRENT_CURSOR_POSITION, "\x1b[s");
        assert_eq!(SCOSC, "\x1b[s");
        assert_eq!(SAVE_CURSOR_POSITION, "\x1b[s");
        assert_eq!(RESTORE_CURRENT_CURSOR_POSITION, "\x1b[u");
        assert_eq!(SCORC, "\x1b[u");
        assert_eq!(RESTORE_CURSOR_POSITION, "\x1b[u");
        assert_eq!(REQUEST_CURSOR_POSITION, "\x1b[6n");
        assert_eq!(REQUEST_EXTENDED_CURSOR_POSITION, "\x1b[?6n");
        assert_eq!(REVERSE_INDEX, "\x1bM");
        assert_eq!(INDEX, "\x1bD");
    }

    #[test]
    fn test_cursor_move_functions() {
        assert_eq!(cursor_up(1), "\x1b[A");
        assert_eq!(cursor_up(0), "\x1b[A");
        assert_eq!(cursor_up(3), "\x1b[3A");
        assert_eq!(cuu(3), "\x1b[3A");
        assert_eq!(CUU1, "\x1b[A");
        assert_eq!(CURSOR_UP1, "\x1b[A");
        assert_eq!(cursor_down(2), "\x1b[2B");
        assert_eq!(cud(2), "\x1b[2B");
        assert_eq!(CUD1, "\x1b[B");
        assert_eq!(CURSOR_DOWN1, "\x1b[B");
        assert_eq!(cursor_forward(4), "\x1b[4C");
        assert_eq!(cuf(4), "\x1b[4C");
        assert_eq!(CUF1, "\x1b[C");
        assert_eq!(cursor_right(4), "\x1b[4C");
        assert_eq!(CURSOR_RIGHT1, "\x1b[C");
        assert_eq!(cursor_backward(5), "\x1b[5D");
        assert_eq!(cub(5), "\x1b[5D");
        assert_eq!(CUB1, "\x1b[D");
        assert_eq!(cursor_left(5), "\x1b[5D");
        assert_eq!(CURSOR_LEFT1, "\x1b[D");
        assert_eq!(cursor_next_line(2), "\x1b[2E");
        assert_eq!(cnl(2), "\x1b[2E");
        assert_eq!(cursor_previous_line(2), "\x1b[2F");
        assert_eq!(cpl(2), "\x1b[2F");
        assert_eq!(cursor_horizontal_forward_tab(3), "\x1b[3I");
        assert_eq!(cht(3), "\x1b[3I");
        assert_eq!(cursor_backward_tab(3), "\x1b[3Z");
        assert_eq!(cbt(3), "\x1b[3Z");
        assert_eq!(erase_character(3), "\x1b[3X");
        assert_eq!(ech(3), "\x1b[3X");
        assert_eq!(erase_character(1), "\x1b[X");
        assert_eq!(cursor_horizontal_absolute(4), "\x1b[4G");
        assert_eq!(cha(4), "\x1b[4G");
        assert_eq!(cursor_horizontal_absolute(0), "\x1b[G");
    }

    #[test]
    fn test_cursor_position_functions() {
        assert_eq!(cursor_position(1, 1), "\x1b[H");
        assert_eq!(cursor_position(2, 5), "\x1b[5;2H");
        assert_eq!(cup(2, 5), "\x1b[5;2H");
        assert_eq!(CURSOR_HOME_POSITION, "\x1b[H");
        assert_eq!(set_cursor_position(1, 1), "\x1b[1;1H");
        assert_eq!(set_cursor_position(0, 0), "\x1b[H");
        assert_eq!(set_cursor_position(3, 0), "\x1b[;3H");
        assert_eq!(HOME_CURSOR_POSITION, "\x1b[H");
        assert_eq!(move_cursor(3, 0), "\x1b[;3H");
        assert_eq!(CURSOR_ORIGIN, "\x1b[1;1H");
        assert_eq!(MOVE_CURSOR_ORIGIN, "\x1b[1;1H");
        assert_eq!(horizontal_vertical_position(2, 5), "\x1b[5;2f");
        assert_eq!(hvp(2, 5), "\x1b[5;2f");
        assert_eq!(HORIZONTAL_VERTICAL_HOME_POSITION, "\x1b[f");
        assert_eq!(horizontal_vertical_position(1, 1), "\x1b[1;1f");
        assert_eq!(vertical_position_absolute(5), "\x1b[5d");
        assert_eq!(vpa(5), "\x1b[5d");
        assert_eq!(vertical_position_absolute(0), "\x1b[d");
        assert_eq!(vertical_position_relative(3), "\x1b[3e");
        assert_eq!(vpr(3), "\x1b[3e");
        assert_eq!(vertical_position_relative(1), "\x1b[e");
        assert_eq!(horizontal_position_absolute(5), "\x1b[5`");
        assert_eq!(hpa(5), "\x1b[5`");
        assert_eq!(horizontal_position_absolute(0), "\x1b[`");
        assert_eq!(horizontal_position_relative(3), "\x1b[3a");
        assert_eq!(hpr(3), "\x1b[3a");
        assert_eq!(horizontal_position_relative(0), "\x1b[a");
        assert_eq!(horizontal_position_relative(1), "\x1b[1a");
    }

    #[test]
    fn test_cursor_style() {
        assert_eq!(set_cursor_style(1), "\x1b[1 q");
        assert_eq!(set_cursor_style(0), "\x1b[0 q");
        assert_eq!(set_cursor_style(-1), "\x1b[0 q");
        assert_eq!(set_cursor_style(6), "\x1b[6 q");
        assert_eq!(decscusr(6), "\x1b[6 q");
        assert_eq!(set_pointer_shape("default"), "\x1b]22;default\x07");
        assert_eq!(set_pointer_shape("text"), "\x1b]22;text\x07");
    }
}
