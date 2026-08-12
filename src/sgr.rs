//! Cleanroom Rust port of upstream Go source file: `ansi/sgr.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! SGR (Select Graphic Rendition) sequence helpers: build `CSI ... m`
//! sequences from SGR attribute codes.
//! </public-docs>

/// Attr is a SGR (Select Graphic Rendition) style attribute.
///
/// See: <https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters>
pub type Attr = i32;

/// SGR attribute: reset all attributes. `0`.
pub const ATTR_RESET: Attr = 0;
/// SGR attribute: bold. `1`.
pub const ATTR_BOLD: Attr = 1;
/// SGR attribute: faint/dim. `2`.
pub const ATTR_FAINT: Attr = 2;
/// SGR attribute: italic. `3`.
pub const ATTR_ITALIC: Attr = 3;
/// SGR attribute: underline. `4`.
pub const ATTR_UNDERLINE: Attr = 4;
/// SGR attribute: blink. `5`.
pub const ATTR_BLINK: Attr = 5;
/// SGR attribute: rapid blink. `6`.
pub const ATTR_RAPID_BLINK: Attr = 6;
/// SGR attribute: reverse video. `7`.
pub const ATTR_REVERSE: Attr = 7;
/// SGR attribute: conceal. `8`.
pub const ATTR_CONCEAL: Attr = 8;
/// SGR attribute: strikethrough. `9`.
pub const ATTR_STRIKETHROUGH: Attr = 9;
/// SGR attribute: normal intensity. `22`.
pub const ATTR_NORMAL_INTENSITY: Attr = 22;
/// SGR attribute: no italic. `23`.
pub const ATTR_NO_ITALIC: Attr = 23;
/// SGR attribute: no underline. `24`.
pub const ATTR_NO_UNDERLINE: Attr = 24;
/// SGR attribute: no blink. `25`.
pub const ATTR_NO_BLINK: Attr = 25;
/// SGR attribute: no reverse. `27`.
pub const ATTR_NO_REVERSE: Attr = 27;
/// SGR attribute: no conceal. `28`.
pub const ATTR_NO_CONCEAL: Attr = 28;
/// SGR attribute: no strikethrough. `29`.
pub const ATTR_NO_STRIKETHROUGH: Attr = 29;
/// SGR attribute: black foreground color. `30`.
pub const ATTR_BLACK_FOREGROUND_COLOR: Attr = 30;
/// SGR attribute: red foreground color. `31`.
pub const ATTR_RED_FOREGROUND_COLOR: Attr = 31;
/// SGR attribute: green foreground color. `32`.
pub const ATTR_GREEN_FOREGROUND_COLOR: Attr = 32;
/// SGR attribute: yellow foreground color. `33`.
pub const ATTR_YELLOW_FOREGROUND_COLOR: Attr = 33;
/// SGR attribute: blue foreground color. `34`.
pub const ATTR_BLUE_FOREGROUND_COLOR: Attr = 34;
/// SGR attribute: magenta foreground color. `35`.
pub const ATTR_MAGENTA_FOREGROUND_COLOR: Attr = 35;
/// SGR attribute: cyan foreground color. `36`.
pub const ATTR_CYAN_FOREGROUND_COLOR: Attr = 36;
/// SGR attribute: white foreground color. `37`.
pub const ATTR_WHITE_FOREGROUND_COLOR: Attr = 37;
/// SGR attribute: extended foreground color introducer. `38`.
pub const ATTR_EXTENDED_FOREGROUND_COLOR: Attr = 38;
/// SGR attribute: default foreground color. `39`.
pub const ATTR_DEFAULT_FOREGROUND_COLOR: Attr = 39;
/// SGR attribute: black background color. `40`.
pub const ATTR_BLACK_BACKGROUND_COLOR: Attr = 40;
/// SGR attribute: red background color. `41`.
pub const ATTR_RED_BACKGROUND_COLOR: Attr = 41;
/// SGR attribute: green background color. `42`.
pub const ATTR_GREEN_BACKGROUND_COLOR: Attr = 42;
/// SGR attribute: yellow background color. `43`.
pub const ATTR_YELLOW_BACKGROUND_COLOR: Attr = 43;
/// SGR attribute: blue background color. `44`.
pub const ATTR_BLUE_BACKGROUND_COLOR: Attr = 44;
/// SGR attribute: magenta background color. `45`.
pub const ATTR_MAGENTA_BACKGROUND_COLOR: Attr = 45;
/// SGR attribute: cyan background color. `46`.
pub const ATTR_CYAN_BACKGROUND_COLOR: Attr = 46;
/// SGR attribute: white background color. `47`.
pub const ATTR_WHITE_BACKGROUND_COLOR: Attr = 47;
/// SGR attribute: extended background color introducer. `48`.
pub const ATTR_EXTENDED_BACKGROUND_COLOR: Attr = 48;
/// SGR attribute: default background color. `49`.
pub const ATTR_DEFAULT_BACKGROUND_COLOR: Attr = 49;
/// SGR attribute: extended underline color introducer. `58`.
pub const ATTR_EXTENDED_UNDERLINE_COLOR: Attr = 58;
/// SGR attribute: default underline color. `59`.
pub const ATTR_DEFAULT_UNDERLINE_COLOR: Attr = 59;
/// SGR attribute: bright black foreground color. `90`.
pub const ATTR_BRIGHT_BLACK_FOREGROUND_COLOR: Attr = 90;
/// SGR attribute: bright red foreground color. `91`.
pub const ATTR_BRIGHT_RED_FOREGROUND_COLOR: Attr = 91;
/// SGR attribute: bright green foreground color. `92`.
pub const ATTR_BRIGHT_GREEN_FOREGROUND_COLOR: Attr = 92;
/// SGR attribute: bright yellow foreground color. `93`.
pub const ATTR_BRIGHT_YELLOW_FOREGROUND_COLOR: Attr = 93;
/// SGR attribute: bright blue foreground color. `94`.
pub const ATTR_BRIGHT_BLUE_FOREGROUND_COLOR: Attr = 94;
/// SGR attribute: bright magenta foreground color. `95`.
pub const ATTR_BRIGHT_MAGENTA_FOREGROUND_COLOR: Attr = 95;
/// SGR attribute: bright cyan foreground color. `96`.
pub const ATTR_BRIGHT_CYAN_FOREGROUND_COLOR: Attr = 96;
/// SGR attribute: bright white foreground color. `97`.
pub const ATTR_BRIGHT_WHITE_FOREGROUND_COLOR: Attr = 97;
/// SGR attribute: bright black background color. `100`.
pub const ATTR_BRIGHT_BLACK_BACKGROUND_COLOR: Attr = 100;
/// SGR attribute: bright red background color. `101`.
pub const ATTR_BRIGHT_RED_BACKGROUND_COLOR: Attr = 101;
/// SGR attribute: bright green background color. `102`.
pub const ATTR_BRIGHT_GREEN_BACKGROUND_COLOR: Attr = 102;
/// SGR attribute: bright yellow background color. `103`.
pub const ATTR_BRIGHT_YELLOW_BACKGROUND_COLOR: Attr = 103;
/// SGR attribute: bright blue background color. `104`.
pub const ATTR_BRIGHT_BLUE_BACKGROUND_COLOR: Attr = 104;
/// SGR attribute: bright magenta background color. `105`.
pub const ATTR_BRIGHT_MAGENTA_BACKGROUND_COLOR: Attr = 105;
/// SGR attribute: bright cyan background color. `106`.
pub const ATTR_BRIGHT_CYAN_BACKGROUND_COLOR: Attr = 106;
/// SGR attribute: bright white background color. `107`.
pub const ATTR_BRIGHT_WHITE_BACKGROUND_COLOR: Attr = 107;
/// SGR attribute: RGB color introducer. `2`.
pub const ATTR_RGB_COLOR_INTRODUCER: Attr = 2;
/// SGR attribute: extended color introducer. `5`.
pub const ATTR_EXTENDED_COLOR_INTRODUCER: Attr = 5;

// Deprecated: use `ATTR_*` constants instead.
/// SGR attribute: reset all attributes. `0`.
pub const RESET_ATTR: Attr = ATTR_RESET;
/// SGR attribute: bold. `1`.
pub const BOLD_ATTR: Attr = ATTR_BOLD;
/// SGR attribute: faint/dim. `2`.
pub const FAINT_ATTR: Attr = ATTR_FAINT;
/// SGR attribute: italic. `3`.
pub const ITALIC_ATTR: Attr = ATTR_ITALIC;
/// SGR attribute: underline. `4`.
pub const UNDERLINE_ATTR: Attr = ATTR_UNDERLINE;
/// SGR attribute: slow blink. `5`.
pub const SLOW_BLINK_ATTR: Attr = ATTR_BLINK;
/// SGR attribute: rapid blink. `6`.
pub const RAPID_BLINK_ATTR: Attr = ATTR_RAPID_BLINK;
/// SGR attribute: reverse video. `7`.
pub const REVERSE_ATTR: Attr = ATTR_REVERSE;
/// SGR attribute: conceal. `8`.
pub const CONCEAL_ATTR: Attr = ATTR_CONCEAL;
/// SGR attribute: strikethrough. `9`.
pub const STRIKETHROUGH_ATTR: Attr = ATTR_STRIKETHROUGH;
/// SGR attribute: normal intensity. `22`.
pub const NORMAL_INTENSITY_ATTR: Attr = ATTR_NORMAL_INTENSITY;
/// SGR attribute: no italic. `23`.
pub const NO_ITALIC_ATTR: Attr = ATTR_NO_ITALIC;
/// SGR attribute: no underline. `24`.
pub const NO_UNDERLINE_ATTR: Attr = ATTR_NO_UNDERLINE;
/// SGR attribute: no blink. `25`.
pub const NO_BLINK_ATTR: Attr = ATTR_NO_BLINK;
/// SGR attribute: no reverse. `27`.
pub const NO_REVERSE_ATTR: Attr = ATTR_NO_REVERSE;
/// SGR attribute: no conceal. `28`.
pub const NO_CONCEAL_ATTR: Attr = ATTR_NO_CONCEAL;
/// SGR attribute: no strikethrough. `29`.
pub const NO_STRIKETHROUGH_ATTR: Attr = ATTR_NO_STRIKETHROUGH;
/// SGR attribute: black foreground color. `30`.
pub const BLACK_FOREGROUND_COLOR_ATTR: Attr = ATTR_BLACK_FOREGROUND_COLOR;
/// SGR attribute: red foreground color. `31`.
pub const RED_FOREGROUND_COLOR_ATTR: Attr = ATTR_RED_FOREGROUND_COLOR;
/// SGR attribute: green foreground color. `32`.
pub const GREEN_FOREGROUND_COLOR_ATTR: Attr = ATTR_GREEN_FOREGROUND_COLOR;
/// SGR attribute: yellow foreground color. `33`.
pub const YELLOW_FOREGROUND_COLOR_ATTR: Attr = ATTR_YELLOW_FOREGROUND_COLOR;
/// SGR attribute: blue foreground color. `34`.
pub const BLUE_FOREGROUND_COLOR_ATTR: Attr = ATTR_BLUE_FOREGROUND_COLOR;
/// SGR attribute: magenta foreground color. `35`.
pub const MAGENTA_FOREGROUND_COLOR_ATTR: Attr = ATTR_MAGENTA_FOREGROUND_COLOR;
/// SGR attribute: cyan foreground color. `36`.
pub const CYAN_FOREGROUND_COLOR_ATTR: Attr = ATTR_CYAN_FOREGROUND_COLOR;
/// SGR attribute: white foreground color. `37`.
pub const WHITE_FOREGROUND_COLOR_ATTR: Attr = ATTR_WHITE_FOREGROUND_COLOR;
/// SGR attribute: extended foreground color introducer. `38`.
pub const EXTENDED_FOREGROUND_COLOR_ATTR: Attr = ATTR_EXTENDED_FOREGROUND_COLOR;
/// SGR attribute: default foreground color. `39`.
pub const DEFAULT_FOREGROUND_COLOR_ATTR: Attr = ATTR_DEFAULT_FOREGROUND_COLOR;
/// SGR attribute: black background color. `40`.
pub const BLACK_BACKGROUND_COLOR_ATTR: Attr = ATTR_BLACK_BACKGROUND_COLOR;
/// SGR attribute: red background color. `41`.
pub const RED_BACKGROUND_COLOR_ATTR: Attr = ATTR_RED_BACKGROUND_COLOR;
/// SGR attribute: green background color. `42`.
pub const GREEN_BACKGROUND_COLOR_ATTR: Attr = ATTR_GREEN_BACKGROUND_COLOR;
/// SGR attribute: yellow background color. `43`.
pub const YELLOW_BACKGROUND_COLOR_ATTR: Attr = ATTR_YELLOW_BACKGROUND_COLOR;
/// SGR attribute: blue background color. `44`.
pub const BLUE_BACKGROUND_COLOR_ATTR: Attr = ATTR_BLUE_BACKGROUND_COLOR;
/// SGR attribute: magenta background color. `45`.
pub const MAGENTA_BACKGROUND_COLOR_ATTR: Attr = ATTR_MAGENTA_BACKGROUND_COLOR;
/// SGR attribute: cyan background color. `46`.
pub const CYAN_BACKGROUND_COLOR_ATTR: Attr = ATTR_CYAN_BACKGROUND_COLOR;
/// SGR attribute: white background color. `47`.
pub const WHITE_BACKGROUND_COLOR_ATTR: Attr = ATTR_WHITE_BACKGROUND_COLOR;
/// SGR attribute: extended background color introducer. `48`.
pub const EXTENDED_BACKGROUND_COLOR_ATTR: Attr = ATTR_EXTENDED_BACKGROUND_COLOR;
/// SGR attribute: default background color. `49`.
pub const DEFAULT_BACKGROUND_COLOR_ATTR: Attr = ATTR_DEFAULT_BACKGROUND_COLOR;
/// SGR attribute: extended underline color introducer. `58`.
pub const EXTENDED_UNDERLINE_COLOR_ATTR: Attr = ATTR_EXTENDED_UNDERLINE_COLOR;
/// SGR attribute: default underline color. `59`.
pub const DEFAULT_UNDERLINE_COLOR_ATTR: Attr = ATTR_DEFAULT_UNDERLINE_COLOR;
/// SGR attribute: bright black foreground color. `90`.
pub const BRIGHT_BLACK_FOREGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_BLACK_FOREGROUND_COLOR;
/// SGR attribute: bright red foreground color. `91`.
pub const BRIGHT_RED_FOREGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_RED_FOREGROUND_COLOR;
/// SGR attribute: bright green foreground color. `92`.
pub const BRIGHT_GREEN_FOREGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_GREEN_FOREGROUND_COLOR;
/// SGR attribute: bright yellow foreground color. `93`.
pub const BRIGHT_YELLOW_FOREGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_YELLOW_FOREGROUND_COLOR;
/// SGR attribute: bright blue foreground color. `94`.
pub const BRIGHT_BLUE_FOREGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_BLUE_FOREGROUND_COLOR;
/// SGR attribute: bright magenta foreground color. `95`.
pub const BRIGHT_MAGENTA_FOREGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_MAGENTA_FOREGROUND_COLOR;
/// SGR attribute: bright cyan foreground color. `96`.
pub const BRIGHT_CYAN_FOREGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_CYAN_FOREGROUND_COLOR;
/// SGR attribute: bright white foreground color. `97`.
pub const BRIGHT_WHITE_FOREGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_WHITE_FOREGROUND_COLOR;
/// SGR attribute: bright black background color. `100`.
pub const BRIGHT_BLACK_BACKGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_BLACK_BACKGROUND_COLOR;
/// SGR attribute: bright red background color. `101`.
pub const BRIGHT_RED_BACKGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_RED_BACKGROUND_COLOR;
/// SGR attribute: bright green background color. `102`.
pub const BRIGHT_GREEN_BACKGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_GREEN_BACKGROUND_COLOR;
/// SGR attribute: bright yellow background color. `103`.
pub const BRIGHT_YELLOW_BACKGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_YELLOW_BACKGROUND_COLOR;
/// SGR attribute: bright blue background color. `104`.
pub const BRIGHT_BLUE_BACKGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_BLUE_BACKGROUND_COLOR;
/// SGR attribute: bright magenta background color. `105`.
pub const BRIGHT_MAGENTA_BACKGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_MAGENTA_BACKGROUND_COLOR;
/// SGR attribute: bright cyan background color. `106`.
pub const BRIGHT_CYAN_BACKGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_CYAN_BACKGROUND_COLOR;
/// SGR attribute: bright white background color. `107`.
pub const BRIGHT_WHITE_BACKGROUND_COLOR_ATTR: Attr = ATTR_BRIGHT_WHITE_BACKGROUND_COLOR;
/// SGR attribute: RGB color introducer. `2`.
pub const RGB_COLOR_INTRODUCER_ATTR: Attr = ATTR_RGB_COLOR_INTRODUCER;
/// SGR attribute: extended color introducer. `5`.
pub const EXTENDED_COLOR_INTRODUCER_ATTR: Attr = ATTR_EXTENDED_COLOR_INTRODUCER;

/// SelectGraphicRendition (SGR) is a command that sets display attributes.
///
/// Default is 0.
///
/// `CSI Ps ; Ps ... m`
///
/// See: <https://vt100.net/docs/vt510-rm/SGR.html>
///
/// Ports upstream `SelectGraphicRendition(ps ...Attr)`. Upstream renders each
/// attribute through the `attrStrings` table, which maps every known code to
/// its own decimal representation (an identity mapping), and falls back to the
/// decimal representation of the code itself (`0` when negative). Both paths
/// produce identical output, so this port renders the decimal form directly.
pub fn select_graphic_rendition(ps: &[Attr]) -> String {
    if ps.is_empty() {
        return crate::style::RESET_STYLE.to_string();
    }
    let attrs: Vec<String> = ps.iter().map(|a| attr_string(*a)).collect();
    format!("\x1b[{}m", attrs.join(";"))
}

/// SGR is an alias for [select_graphic_rendition].
pub fn sgr(ps: &[Attr]) -> String {
    select_graphic_rendition(ps)
}

/// Returns the decimal SGR parameter for an attribute code, mirroring the
/// upstream `attrStrings` lookup (identity) with the negative-to-zero fallback.
fn attr_string(a: Attr) -> String {
    if a < 0 {
        "0".to_string()
    } else {
        a.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_graphic_rendition() {
        assert_eq!(select_graphic_rendition(&[]), "\x1b[m");
        assert_eq!(select_graphic_rendition(&[BOLD_ATTR]), "\x1b[1m");
        assert_eq!(
            select_graphic_rendition(&[BOLD_ATTR, ITALIC_ATTR, UNDERLINE_ATTR]),
            "\x1b[1;3;4m"
        );
        assert_eq!(
            select_graphic_rendition(&[RED_FOREGROUND_COLOR_ATTR, BOLD_ATTR]),
            "\x1b[31;1m"
        );
        assert_eq!(
            select_graphic_rendition(&[BLUE_BACKGROUND_COLOR_ATTR, BOLD_ATTR]),
            "\x1b[44;1m"
        );
        assert_eq!(
            select_graphic_rendition(&[
                BRIGHT_RED_FOREGROUND_COLOR_ATTR,
                BRIGHT_BLUE_BACKGROUND_COLOR_ATTR
            ]),
            "\x1b[91;104m"
        );
        assert_eq!(select_graphic_rendition(&[RESET_ATTR]), "\x1b[0m");
        assert_eq!(select_graphic_rendition(&[-1]), "\x1b[0m");
        assert_eq!(select_graphic_rendition(&[99]), "\x1b[99m");
        assert_eq!(
            select_graphic_rendition(&[BOLD_ATTR, 99, ITALIC_ATTR]),
            "\x1b[1;99;3m"
        );
        assert_eq!(
            select_graphic_rendition(&[
                BOLD_ATTR,
                FAINT_ATTR,
                ITALIC_ATTR,
                UNDERLINE_ATTR,
                SLOW_BLINK_ATTR,
                REVERSE_ATTR,
                CONCEAL_ATTR,
                STRIKETHROUGH_ATTR,
            ]),
            "\x1b[1;2;3;4;5;7;8;9m"
        );
        assert_eq!(
            select_graphic_rendition(&[
                DEFAULT_FOREGROUND_COLOR_ATTR,
                DEFAULT_BACKGROUND_COLOR_ATTR,
                DEFAULT_UNDERLINE_COLOR_ATTR,
            ]),
            "\x1b[39;49;59m"
        );
        assert_eq!(
            select_graphic_rendition(&[
                EXTENDED_FOREGROUND_COLOR_ATTR,
                EXTENDED_BACKGROUND_COLOR_ATTR,
                EXTENDED_UNDERLINE_COLOR_ATTR,
            ]),
            "\x1b[38;48;58m"
        );
    }

    #[test]
    fn test_sgr_alias() {
        assert_eq!(sgr(&[]), select_graphic_rendition(&[]));
        assert_eq!(sgr(&[BOLD_ATTR]), select_graphic_rendition(&[BOLD_ATTR]));
        assert_eq!(
            sgr(&[
                BOLD_ATTR,
                RED_FOREGROUND_COLOR_ATTR,
                BLUE_BACKGROUND_COLOR_ATTR
            ]),
            select_graphic_rendition(&[
                BOLD_ATTR,
                RED_FOREGROUND_COLOR_ATTR,
                BLUE_BACKGROUND_COLOR_ATTR
            ])
        );
    }
}
