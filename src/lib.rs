//! Cleanroom Rust port of upstream Go source file: `ansi/doc.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! An ANSI escape-code library for Go, ported to Rust. Provides SGR styles,
//! string width measurement, wrapping, truncation, hyperlinks, color
//! conversion, Kitty keyboard protocol sequences, terminal modes, cursor
//! control, mouse events, and progress bar sequences.
//! </public-docs>

#![deny(unsafe_code)]

pub mod background;
pub mod color;
pub mod cursor;
pub mod hyperlink;
pub mod kitty;
pub mod method;
pub mod mode;
pub mod mouse;
pub mod parser;
pub mod progress;
pub mod sgr;
pub mod screen;
pub mod style;
pub mod util;
pub mod width;
pub mod wrap;

pub use background::{
    HexColor, REQUEST_BACKGROUND_COLOR, REQUEST_CURSOR_COLOR, REQUEST_FOREGROUND_COLOR,
    RESET_BACKGROUND_COLOR, RESET_CURSOR_COLOR, RESET_FOREGROUND_COLOR, XRGBAColor,
    XRGBColor, set_background_color, set_cursor_color, set_foreground_color,
};
pub use color::{
    ansi256_to_16, convert_16, convert_256, BasicColor, IndexedColor, RGBColor, BLUE,
    BRIGHT_BLACK, BRIGHT_BLUE, BRIGHT_CYAN, BRIGHT_GREEN, BRIGHT_MAGENTA, BRIGHT_RED,
    BRIGHT_WHITE, BRIGHT_YELLOW, BLACK, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW,
};
pub use cursor::{
    cursor_backward, cursor_down, cursor_forward, cursor_horizontal_absolute,
    cursor_horizontal_forward_tab, cursor_left, cursor_next_line, cursor_position,
    cursor_previous_line, cursor_right, cursor_up, erase_character, horizontal_position_absolute,
    horizontal_position_relative, horizontal_vertical_position, move_cursor,
    set_cursor_position, set_cursor_style, set_pointer_shape, vertical_position_absolute,
    vertical_position_relative, CUB1, CUD1, CUF1, CUU1, CURSOR_HOME_POSITION,
    RESTORE_CURSOR, SAVE_CURSOR,
};
pub use hyperlink::{reset_hyperlink, set_hyperlink};
pub use kitty::{
    kitty_keyboard, pop_kitty_keyboard, push_kitty_keyboard, KITTY_ALL_FLAGS,
    KITTY_DISAMBIGUATE_ESCAPE_CODES, KITTY_REPORT_ALL_KEYS_AS_ESCAPE_CODES,
    KITTY_REPORT_ALTERNATE_KEYS, KITTY_REPORT_ASSOCIATED_KEYS, KITTY_REPORT_EVENT_TYPES,
};
pub use method::WidthMethod;
pub use parser::{
    decode_sequence, decode_sequence_wc, get_parser, has_apc_prefix,
    has_csi_prefix, has_dcs_prefix, has_esc_prefix, has_osc_prefix, has_pm_prefix,
    has_sos_prefix, has_st_prefix, new_parser, put_parser, APC, APC_STRING_STATE, BEL, BS, CAN,
    CSI, CSI_ENTRY_STATE, CSI_INTERMEDIATE_STATE, CSI_PARAM_STATE, Cmd, DEL, DCS,
    DCS_ENTRY_STATE, DCS_INTERMEDIATE_STATE, DCS_PARAM_STATE, DCS_STRING_STATE, ESC,
    ESCAPE_INTERMEDIATE_STATE, ESCAPE_STATE, ESCAPE_STATE_DECODE, GROUND_STATE, HAS_MORE_FLAG,
    Handler, INTERMED_STATE, LF, MAX_PARAMS_SIZE, MISSING_COMMAND, MISSING_PARAM, NORMAL_STATE,
    OSC, OSC_STRING_STATE, PARAM_MASK, PARAMS_STATE, PM, PM_STRING_STATE, PREFIX_STATE, Param,
    Params, Parser, SOS, SOS_STRING_STATE, SP, ST, STRING_STATE, SUB, UTF8_STATE,
};
pub use mode::{
    decrst, decset, decrpm, decrqm, report_mode, request_mode, reset_mode, rm, set_mode, sm,
    ANSIMode, DECMode, Mode, ModeSetting, Modes, BRACKETED_PASTE_MODE, DECAWM, DECBKM, DECCKM,
    DECLRMM, DECNKM, DECOM, DECTCEM, HIDE_CURSOR, IRM, KAM, LNM, MODE_ALT_SCREEN,
    MODE_ALT_SCREEN_SAVE_CURSOR, MODE_AUTO_WRAP, MODE_BACKARROW_KEY, MODE_BRACKETED_PASTE,
    MODE_CURSOR_KEYS, MODE_FOCUS_EVENT, MODE_IN_BAND_RESIZE, MODE_INSERT_REPLACE,
    MODE_KEYBOARD_ACTION, MODE_LEFT_RIGHT_MARGIN, MODE_LIGHT_DARK, MODE_LINE_FEED_NEW_LINE,
    MODE_MOUSE_ANY_EVENT, MODE_MOUSE_BUTTON_EVENT, MODE_MOUSE_EXT_SGR, MODE_MOUSE_EXT_SGR_PIXEL,
    MODE_MOUSE_EXT_URXVT, MODE_MOUSE_EXT_UTF8, MODE_MOUSE_HIGHLIGHT, MODE_MOUSE_NORMAL,
    MODE_MOUSE_X10, MODE_NUMERIC_KEYPAD, MODE_ORIGIN, MODE_SAVE_CURSOR, MODE_SEND_RECEIVE,
    MODE_SYNCHRONIZED_OUTPUT, MODE_TEXT_CURSOR_ENABLE, MODE_UNICODE_CORE, MODE_WIN32_INPUT,
    RESET_MODE_ALT_SCREEN, RESET_MODE_BRACKETED_PASTE, RESET_MODE_MOUSE_ANY_EVENT,
    RESET_MODE_MOUSE_BUTTON_EVENT, RESET_MODE_MOUSE_EXT_SGR, RESET_MODE_MOUSE_EXT_SGR_PIXEL,
    RESET_MODE_MOUSE_EXT_URXVT, RESET_MODE_MOUSE_NORMAL, RESET_MODE_MOUSE_X10, SET_MODE_ALT_SCREEN,
    SET_MODE_BRACKETED_PASTE, SET_MODE_MOUSE_ANY_EVENT, SET_MODE_MOUSE_BUTTON_EVENT,
    SET_MODE_MOUSE_EXT_SGR, SET_MODE_MOUSE_EXT_SGR_PIXEL, SET_MODE_MOUSE_EXT_URXVT,
    SET_MODE_MOUSE_NORMAL, SET_MODE_MOUSE_X10, SHOW_CURSOR, SRM,
};
pub use mouse::{
    encode_mouse_button, mouse_sgr, mouse_x10, MouseButton, MOUSE_BACKWARD, MOUSE_BUTTON_1,
    MOUSE_BUTTON_10, MOUSE_BUTTON_11, MOUSE_BUTTON_2, MOUSE_BUTTON_3, MOUSE_BUTTON_4,
    MOUSE_BUTTON_5, MOUSE_BUTTON_6, MOUSE_BUTTON_7, MOUSE_BUTTON_8, MOUSE_BUTTON_9,
    MOUSE_FORWARD, MOUSE_LEFT, MOUSE_MIDDLE, MOUSE_NONE, MOUSE_RELEASE, MOUSE_RIGHT,
    MOUSE_WHEEL_DOWN, MOUSE_WHEEL_LEFT, MOUSE_WHEEL_RIGHT, MOUSE_WHEEL_UP,
};
pub use progress::{
    set_error_progress_bar, set_progress_bar, set_warning_progress_bar, RESET_PROGRESS_BAR,
    SET_INDETERMINATE_PROGRESS_BAR,
};
pub use screen::{
    delete_character, delete_line, dch, decstbm, decslrm, deccir, dectabsr, decrqpsr, dl, ed,
    el, erase_display, erase_line, ich, il, insert_character, insert_line, pan_down, pan_up,
    repeat_previous_character, rep, request_presentation_state_report, scroll_down, scroll_up,
    sd, su, set_left_right_margins, set_scrolling_region, set_top_bottom_margins, tab_clear,
    tab_stop_report, tbc, DECST8C, ERASE_ENTIRE_DISPLAY, ERASE_ENTIRE_LINE,
    ERASE_ENTIRE_SCREEN, ERASE_LINE_LEFT, ERASE_LINE_RIGHT, ERASE_SCREEN_ABOVE,
    ERASE_SCREEN_BELOW, HORIZONTAL_TAB_SET, SET_TAB_EVERY_8_COLUMNS,
};
pub use sgr::{
    select_graphic_rendition, sgr, ATTR_BLINK, ATTR_BOLD, ATTR_CONCEAL,
    ATTR_CYAN_BACKGROUND_COLOR, ATTR_CYAN_FOREGROUND_COLOR,
    ATTR_DEFAULT_BACKGROUND_COLOR, ATTR_DEFAULT_FOREGROUND_COLOR, ATTR_DEFAULT_UNDERLINE_COLOR,
    ATTR_EXTENDED_BACKGROUND_COLOR, ATTR_EXTENDED_FOREGROUND_COLOR, ATTR_EXTENDED_UNDERLINE_COLOR,
    ATTR_FAINT, ATTR_ITALIC, ATTR_MAGENTA_BACKGROUND_COLOR, ATTR_MAGENTA_FOREGROUND_COLOR,
    ATTR_NORMAL_INTENSITY, ATTR_NO_BLINK, ATTR_NO_CONCEAL, ATTR_NO_ITALIC, ATTR_NO_REVERSE,
    ATTR_NO_STRIKETHROUGH, ATTR_NO_UNDERLINE, ATTR_RAPID_BLINK, ATTR_RESET, ATTR_REVERSE,
    ATTR_RGB_COLOR_INTRODUCER, ATTR_STRIKETHROUGH, ATTR_UNDERLINE, Attr,
};
pub use style::{Color, Style, Underline, RESET_STYLE};
pub use util::{cut, cut_left, strip, string_width, truncate, truncate_left, x_parse_color};
pub use width::first_grapheme_cluster;
pub use wrap::{hardwrap, hardwrap_wc, wrap, wrap_wc};
