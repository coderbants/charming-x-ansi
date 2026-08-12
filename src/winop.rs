//! Cleanroom Rust port of upstream Go source file: `ansi/winop.go`
//! Upstream Target Tag / Version: `ansi/v0.11.7`
//!
//! <public-docs>
//! Window manipulation sequences (XTWINOPS): resize requests and size
//! reports for the terminal window and cells.
//! </public-docs>

/// ResizeWindowWinOp is a window operation that resizes the terminal
/// window.
///
/// Deprecated: use the constant number directly with [window_op].
pub const RESIZE_WINDOW_WIN_OP: i32 = 4;

/// RequestWindowSizeWinOp is a window operation that requests a report of
/// the size of the terminal window in pixels. The response is in the form:
///
/// ```text
/// CSI 4 ; height ; width t
/// ```
///
/// Deprecated: use the constant number directly with [window_op].
pub const REQUEST_WINDOW_SIZE_WIN_OP: i32 = 14;

/// RequestCellSizeWinOp is a window operation that requests a report of
/// the size of the terminal cell size in pixels. The response is in the form:
///
/// ```text
/// CSI 6 ; height ; width t
/// ```
///
/// Deprecated: use the constant number directly with [window_op].
pub const REQUEST_CELL_SIZE_WIN_OP: i32 = 16;

/// WindowOp (XTWINOPS) is a sequence that manipulates the terminal window.
///
/// ```text
/// CSI Ps ; Ps ; Ps t
/// ```
///
/// Ps is a semicolon-separated list of parameters.
/// See <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h4-Functions-using-CSI-_-ordered-by-the-final-character-lparen-s-rparen:CSI-Ps;Ps;Ps-t.1EB0>
pub fn window_op(p: i32, ps: &[i32]) -> String {
    if p <= 0 {
        return String::new();
    }

    if ps.is_empty() {
        return format!("\x1b[{p}t");
    }

    let mut params: Vec<String> = Vec::with_capacity(ps.len() + 1);
    params.push(p.to_string());
    for pp in ps {
        if *pp >= 0 {
            params.push(pp.to_string());
        }
    }

    format!("\x1b[{}t", params.join(";"))
}

/// XTWINOPS is an alias for [window_op].
pub fn xtwinops(p: i32, ps: &[i32]) -> String {
    window_op(p, ps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_op() {
        assert_eq!(window_op(4, &[]), "\x1b[4t");
        assert_eq!(window_op(14, &[]), "\x1b[14t");
        assert_eq!(window_op(0, &[]), "");
        assert_eq!(window_op(-1, &[]), "");
        assert_eq!(window_op(4, &[120, 100]), "\x1b[4;120;100t");
        assert_eq!(window_op(4, &[120, -1]), "\x1b[4;120t");
        assert_eq!(xtwinops(6, &[40, 20]), "\x1b[6;40;20t");
    }
}
