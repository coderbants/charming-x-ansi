//! Cleanroom Rust port of upstream Go source file: `ansi/mouse.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! Mouse button encoding and X10/SGR mouse event sequences.
//! </public-docs>

/// MouseButton represents the button that was pressed during a mouse message.
///
/// This is based on X11 mouse button codes:
///
/// 1 = left button, 2 = middle button (pressing the scroll wheel),
/// 3 = right button, 4 = turn scroll wheel up, 5 = turn scroll wheel down,
/// 6 = push scroll wheel left, 7 = push scroll wheel right, 8 = 4th button
/// (aka browser backward button), 9 = 5th button (aka browser forward
/// button), 10, 11. Other buttons are not supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseButton(pub u8);

/// Mouse event button: no button (release).
pub const MOUSE_NONE: MouseButton = MouseButton(0);
/// Mouse event button: button 1.
pub const MOUSE_BUTTON_1: MouseButton = MouseButton(1);
/// Mouse event button: button 2.
pub const MOUSE_BUTTON_2: MouseButton = MouseButton(2);
/// Mouse event button: button 3.
pub const MOUSE_BUTTON_3: MouseButton = MouseButton(3);
/// Mouse event button: button 4.
pub const MOUSE_BUTTON_4: MouseButton = MouseButton(4);
/// Mouse event button: button 5.
pub const MOUSE_BUTTON_5: MouseButton = MouseButton(5);
/// Mouse event button: button 6.
pub const MOUSE_BUTTON_6: MouseButton = MouseButton(6);
/// Mouse event button: button 7.
pub const MOUSE_BUTTON_7: MouseButton = MouseButton(7);
/// Mouse event button: button 8.
pub const MOUSE_BUTTON_8: MouseButton = MouseButton(8);
/// Mouse event button: button 9.
pub const MOUSE_BUTTON_9: MouseButton = MouseButton(9);
/// Mouse event button: button 10.
pub const MOUSE_BUTTON_10: MouseButton = MouseButton(10);
/// Mouse event button: button 11.
pub const MOUSE_BUTTON_11: MouseButton = MouseButton(11);

/// Mouse event button: left button. Alias for [MOUSE_BUTTON_1].
pub const MOUSE_LEFT: MouseButton = MOUSE_BUTTON_1;
/// Mouse event button: middle button (pressing the scroll wheel). Alias for [MOUSE_BUTTON_2].
pub const MOUSE_MIDDLE: MouseButton = MOUSE_BUTTON_2;
/// Mouse event button: right button. Alias for [MOUSE_BUTTON_3].
pub const MOUSE_RIGHT: MouseButton = MOUSE_BUTTON_3;
/// Mouse event button: turn scroll wheel up. Alias for [MOUSE_BUTTON_4].
pub const MOUSE_WHEEL_UP: MouseButton = MOUSE_BUTTON_4;
/// Mouse event button: turn scroll wheel down. Alias for [MOUSE_BUTTON_5].
pub const MOUSE_WHEEL_DOWN: MouseButton = MOUSE_BUTTON_5;
/// Mouse event button: push scroll wheel left. Alias for [MOUSE_BUTTON_6].
pub const MOUSE_WHEEL_LEFT: MouseButton = MOUSE_BUTTON_6;
/// Mouse event button: push scroll wheel right. Alias for [MOUSE_BUTTON_7].
pub const MOUSE_WHEEL_RIGHT: MouseButton = MOUSE_BUTTON_7;
/// Mouse event button: 4th button (aka browser backward button). Alias for [MOUSE_BUTTON_8].
pub const MOUSE_BACKWARD: MouseButton = MOUSE_BUTTON_8;
/// Mouse event button: 5th button (aka browser forward button). Alias for [MOUSE_BUTTON_9].
pub const MOUSE_FORWARD: MouseButton = MOUSE_BUTTON_9;
/// Mouse event button: release event. Alias for [MOUSE_NONE].
pub const MOUSE_RELEASE: MouseButton = MOUSE_NONE;

impl MouseButton {
    /// Returns the string representation of the mouse button. Unknown buttons
    /// return an empty string, mirroring the upstream `String()` method's zero
    /// value.
    pub fn as_str(self) -> &'static str {
        match self {
            MOUSE_NONE => "none",
            MOUSE_LEFT => "left",
            MOUSE_MIDDLE => "middle",
            MOUSE_RIGHT => "right",
            MOUSE_WHEEL_UP => "wheelup",
            MOUSE_WHEEL_DOWN => "wheeldown",
            MOUSE_WHEEL_LEFT => "wheelleft",
            MOUSE_WHEEL_RIGHT => "wheelright",
            MOUSE_BACKWARD => "backward",
            MOUSE_FORWARD => "forward",
            MOUSE_BUTTON_10 => "button10",
            MOUSE_BUTTON_11 => "button11",
            _ => "",
        }
    }
}

/// EncodeMouseButton returns a byte representing a mouse button.
/// The button is a bitmask of the following leftmost values:
///
/// - The first two bits are the button number:
///   0 = left button, wheel up, or button no. 8 aka (backwards)
///   1 = middle button, wheel down, or button no. 9 aka (forwards)
///   2 = right button, wheel left, or button no. 10
///   3 = release event, wheel right, or button no. 11
///
/// - The third bit indicates whether the shift key was pressed.
///
/// - The fourth bit indicates the alt key was pressed.
///
/// - The fifth bit indicates the control key was pressed.
///
/// - The sixth bit indicates motion events. Combined with button number 3, i.e.
///   release event, it represents a drag event.
///
/// - The seventh bit indicates a wheel event.
///
/// - The eighth bit indicates additional buttons.
///
/// If button is [MOUSE_NONE], and motion is false, this returns a release
/// event. If button is undefined, this function returns 0xff.
pub fn encode_mouse_button(b: MouseButton, motion: bool, shift: bool, alt: bool, ctrl: bool) -> u8 {
    // mouse bit shifts
    const BIT_SHIFT: u8 = 0b0000_0100;
    const BIT_ALT: u8 = 0b0000_1000;
    const BIT_CTRL: u8 = 0b0001_0000;
    const BIT_MOTION: u8 = 0b0010_0000;
    const BIT_WHEEL: u8 = 0b0100_0000;
    const BIT_ADD: u8 = 0b1000_0000; // additional buttons 8-11

    const BITS_MASK: u8 = 0b0000_0011;

    let mut m: u8;
    if b == MOUSE_NONE {
        m = BITS_MASK;
    } else if b.0 >= MOUSE_LEFT.0 && b.0 <= MOUSE_RIGHT.0 {
        m = b.0 - MOUSE_LEFT.0;
    } else if b.0 >= MOUSE_WHEEL_UP.0 && b.0 <= MOUSE_WHEEL_RIGHT.0 {
        m = b.0 - MOUSE_WHEEL_UP.0;
        m |= BIT_WHEEL;
    } else if b.0 >= MOUSE_BACKWARD.0 && b.0 <= MOUSE_BUTTON_11.0 {
        m = b.0 - MOUSE_BACKWARD.0;
        m |= BIT_ADD;
    } else {
        m = 0xff; // invalid button
    }

    if shift {
        m |= BIT_SHIFT;
    }
    if alt {
        m |= BIT_ALT;
    }
    if ctrl {
        m |= BIT_CTRL;
    }
    if motion {
        m |= BIT_MOTION;
    }

    m
}

/// x10Offset is the offset for X10 mouse events.
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#Mouse%20Tracking>
const X10_OFFSET: u8 = 32;

/// MouseX10 returns an escape sequence representing a mouse event in X10 mode.
/// Note that this requires the terminal support X10 mouse modes.
///
/// `CSI M Cb Cx Cy`
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#Mouse%20Tracking>
pub fn mouse_x10(b: u8, x: i32, y: i32) -> String {
    let mut s = String::from("\x1b[M");
    s.push(b.wrapping_add(X10_OFFSET) as char);
    s.push((x as u8).wrapping_add(X10_OFFSET + 1) as char);
    s.push((y as u8).wrapping_add(X10_OFFSET + 1) as char);
    s
}

/// MouseSgr returns an escape sequence representing a mouse event in SGR mode.
///
/// `CSI < Cb ; Cx ; Cy M`
/// `CSI < Cb ; Cx ; Cy m` (release)
///
/// See: <https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#Mouse%20Tracking>
pub fn mouse_sgr(b: u8, x: i32, y: i32, release: bool) -> String {
    let s = if release { 'm' } else { 'M' };
    let x = x.abs();
    let y = y.abs();
    format!("\x1b[<{};{};{}{}", b, x + 1, y + 1, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_button_strings() {
        assert_eq!(MOUSE_NONE.as_str(), "none");
        assert_eq!(MOUSE_LEFT.as_str(), "left");
        assert_eq!(MOUSE_MIDDLE.as_str(), "middle");
        assert_eq!(MOUSE_RIGHT.as_str(), "right");
        assert_eq!(MOUSE_WHEEL_UP.as_str(), "wheelup");
        assert_eq!(MOUSE_WHEEL_DOWN.as_str(), "wheeldown");
        assert_eq!(MOUSE_WHEEL_LEFT.as_str(), "wheelleft");
        assert_eq!(MOUSE_WHEEL_RIGHT.as_str(), "wheelright");
        assert_eq!(MOUSE_BACKWARD.as_str(), "backward");
        assert_eq!(MOUSE_FORWARD.as_str(), "forward");
        assert_eq!(MOUSE_BUTTON_10.as_str(), "button10");
        assert_eq!(MOUSE_BUTTON_11.as_str(), "button11");
        assert_eq!(MouseButton(0xff).as_str(), "");
    }

    #[test]
    fn test_encode_mouse_button() {
        let cases: &[(&str, MouseButton, bool, bool, bool, bool, u8)] = &[
            (
                "mouse release",
                MOUSE_NONE,
                false,
                false,
                false,
                false,
                0b0000_0011,
            ),
            (
                "mouse release with ctrl",
                MOUSE_NONE,
                false,
                false,
                false,
                true,
                0b0001_0011,
            ),
            (
                "mouse left",
                MOUSE_LEFT,
                false,
                false,
                false,
                false,
                0b0000_0000,
            ),
            (
                "mouse right",
                MOUSE_RIGHT,
                false,
                false,
                false,
                false,
                0b0000_0010,
            ),
            (
                "mouse wheel up",
                MOUSE_WHEEL_UP,
                false,
                false,
                false,
                false,
                0b0100_0000,
            ),
            (
                "mouse wheel right",
                MOUSE_WHEEL_RIGHT,
                false,
                false,
                false,
                false,
                0b0100_0011,
            ),
            (
                "mouse backward",
                MOUSE_BACKWARD,
                false,
                false,
                false,
                false,
                0b1000_0000,
            ),
            (
                "mouse forward",
                MOUSE_FORWARD,
                false,
                false,
                false,
                false,
                0b1000_0001,
            ),
            (
                "mouse button 10",
                MOUSE_BUTTON_10,
                false,
                false,
                false,
                false,
                0b1000_0010,
            ),
            (
                "mouse button 11",
                MOUSE_BUTTON_11,
                false,
                false,
                false,
                false,
                0b1000_0011,
            ),
            (
                "mouse middle with motion",
                MOUSE_MIDDLE,
                true,
                false,
                false,
                false,
                0b0010_0001,
            ),
            (
                "mouse middle with shift",
                MOUSE_MIDDLE,
                false,
                true,
                false,
                false,
                0b0000_0101,
            ),
            (
                "mouse middle with motion and alt",
                MOUSE_MIDDLE,
                true,
                false,
                true,
                false,
                0b0010_1001,
            ),
            (
                "mouse right with shift, alt, and ctrl",
                MOUSE_RIGHT,
                false,
                true,
                true,
                true,
                0b0001_1110,
            ),
            (
                "mouse button 10 with motion, shift, alt, and ctrl",
                MOUSE_BUTTON_10,
                true,
                true,
                true,
                true,
                0b1011_1110,
            ),
            (
                "mouse left with motion, shift, and ctrl",
                MOUSE_LEFT,
                true,
                true,
                false,
                true,
                0b0011_0100,
            ),
            (
                "invalid mouse button",
                MouseButton(0xff),
                false,
                false,
                false,
                false,
                0b1111_1111,
            ),
            (
                "mouse wheel down with motion",
                MOUSE_WHEEL_DOWN,
                true,
                false,
                false,
                false,
                0b0110_0001,
            ),
            (
                "mouse wheel down with shift and ctrl",
                MOUSE_WHEEL_DOWN,
                false,
                true,
                false,
                true,
                0b0101_0101,
            ),
            (
                "mouse wheel left with alt",
                MOUSE_WHEEL_LEFT,
                false,
                false,
                true,
                false,
                0b0100_1010,
            ),
            (
                "mouse middle with all modifiers",
                MOUSE_MIDDLE,
                true,
                true,
                true,
                true,
                0b0011_1101,
            ),
        ];
        for (name, btn, motion, shift, alt, ctrl, want) in cases {
            assert_eq!(
                encode_mouse_button(*btn, *motion, *shift, *alt, *ctrl),
                *want,
                "{}",
                name
            );
        }
    }

    #[test]
    fn test_mouse_x10() {
        assert_eq!(mouse_x10(0, 0, 0), "\x1b[M !!");
        assert_eq!(mouse_x10(1, 5, 7), "\x1b[M!&(");
    }

    #[test]
    fn test_mouse_sgr() {
        let cases: &[(&str, MouseButton, i32, i32, bool)] = &[
            ("mouse left", MOUSE_LEFT, 0, 0, false),
            ("wheel down", MOUSE_WHEEL_DOWN, 1, 10, false),
            (
                "mouse right with shift, alt, and ctrl",
                MOUSE_RIGHT,
                10,
                1,
                false,
            ),
            ("mouse release", MOUSE_NONE, 5, 5, true),
            (
                "mouse button 10 with motion, shift, alt, and ctrl",
                MOUSE_BUTTON_10,
                10,
                10,
                false,
            ),
            ("mouse wheel up with motion", MOUSE_WHEEL_UP, 15, 15, false),
            (
                "mouse middle with all modifiers",
                MOUSE_MIDDLE,
                20,
                20,
                false,
            ),
            (
                "mouse wheel left at max coordinates",
                MOUSE_WHEEL_LEFT,
                223,
                223,
                false,
            ),
            ("mouse forward release", MOUSE_FORWARD, 100, 100, true),
            (
                "mouse backward with shift and ctrl",
                MOUSE_BACKWARD,
                50,
                50,
                false,
            ),
        ];
        for (name, btn, x, y, release) in cases {
            let b = encode_mouse_button(*btn, false, false, false, false);
            let m = mouse_sgr(b, *x, *y, *release);
            let action = if *release { 'm' } else { 'M' };
            let want = format!("\x1b[<{};{};{}{}", b, x + 1, y + 1, action);
            assert_eq!(m, want, "{}", name);
        }
        assert_eq!(mouse_sgr(0, -5, -2, false), "\x1b[<0;6;3M");
    }
}
