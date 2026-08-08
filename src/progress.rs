//! Cleanroom Rust port of upstream Go source file: `ansi/progress.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! Windows Terminal progress bar sequences (OSC 9;4).
//! </public-docs>

/// ResetProgressBar is a sequence that resets the progress bar to its default
/// state (hidden).
///
/// `OSC 9 ; 4 ; 0 BEL`
///
/// See: <https://learn.microsoft.com/en-us/windows/terminal/tutorials/progress-bar-sequences>
pub const RESET_PROGRESS_BAR: &str = "\x1b]9;4;0\x07";

/// SetProgressBar returns a sequence for setting the progress bar to a
/// specific percentage (0-100) in the "default" state.
///
/// `OSC 9 ; 4 ; 1 Percentage BEL`
///
/// See: <https://learn.microsoft.com/en-us/windows/terminal/tutorials/progress-bar-sequences>
pub fn set_progress_bar(percentage: i32) -> String {
    format!("\x1b]9;4;1;{}\x07", percentage.clamp(0, 100))
}

/// SetErrorProgressBar returns a sequence for setting the progress bar to a
/// specific percentage (0-100) in the "Error" state.
///
/// `OSC 9 ; 4 ; 2 Percentage BEL`
///
/// See: <https://learn.microsoft.com/en-us/windows/terminal/tutorials/progress-bar-sequences>
pub fn set_error_progress_bar(percentage: i32) -> String {
    format!("\x1b]9;4;2;{}\x07", percentage.clamp(0, 100))
}

/// SetIndeterminateProgressBar is a sequence that sets the progress bar to the
/// indeterminate state.
///
/// `OSC 9 ; 4 ; 3 BEL`
///
/// See: <https://learn.microsoft.com/en-us/windows/terminal/tutorials/progress-bar-sequences>
pub const SET_INDETERMINATE_PROGRESS_BAR: &str = "\x1b]9;4;3\x07";

/// SetWarningProgressBar is a sequence that sets the progress bar to the
/// "Warning" state.
///
/// `OSC 9 ; 4 ; 4 Percentage BEL`
///
/// See: <https://learn.microsoft.com/en-us/windows/terminal/tutorials/progress-bar-sequences>
pub fn set_warning_progress_bar(percentage: i32) -> String {
    format!("\x1b]9;4;4;{}\x07", percentage.clamp(0, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_progress_bar() {
        assert_eq!(set_progress_bar(50), "\x1b]9;4;1;50\x07");
        assert_eq!(set_progress_bar(-2), "\x1b]9;4;1;0\x07");
        assert_eq!(set_progress_bar(200), "\x1b]9;4;1;100\x07");
        assert_eq!(RESET_PROGRESS_BAR, "\x1b]9;4;0\x07");
    }

    #[test]
    fn test_set_error_progress_bar() {
        assert_eq!(set_error_progress_bar(50), "\x1b]9;4;2;50\x07");
        assert_eq!(set_error_progress_bar(-2), "\x1b]9;4;2;0\x07");
        assert_eq!(set_error_progress_bar(200), "\x1b]9;4;2;100\x07");
    }

    #[test]
    fn test_set_indeterminate_progress_bar() {
        assert_eq!(SET_INDETERMINATE_PROGRESS_BAR, "\x1b]9;4;3\x07");
    }

    #[test]
    fn test_set_warning_progress_bar() {
        assert_eq!(set_warning_progress_bar(50), "\x1b]9;4;4;50\x07");
        assert_eq!(set_warning_progress_bar(-2), "\x1b]9;4;4;0\x07");
        assert_eq!(set_warning_progress_bar(200), "\x1b]9;4;4;100\x07");
    }
}
