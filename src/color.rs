//! Cleanroom Rust port of upstream Go source file: `ansi/color.go`
//! Upstream Target Tag / Version: `v0.11.7`
//!
//! <public-docs>
//! Terminal color types and palette conversion (xterm-256 and 16-color).
//! </public-docs>

/// BasicColor is a 4-bit ANSI color (0-15).
pub type BasicColor = u8;

/// The ANSI black color. `0`.
pub const BLACK: BasicColor = 0;
/// The ANSI red color. `1`.
pub const RED: BasicColor = 1;
/// The ANSI green color. `2`.
pub const GREEN: BasicColor = 2;
/// The ANSI yellow color. `3`.
pub const YELLOW: BasicColor = 3;
/// The ANSI blue color. `4`.
pub const BLUE: BasicColor = 4;
/// The ANSI magenta color. `5`.
pub const MAGENTA: BasicColor = 5;
/// The ANSI cyan color. `6`.
pub const CYAN: BasicColor = 6;
/// The ANSI white color. `7`.
pub const WHITE: BasicColor = 7;
/// The ANSI bright black color. `8`.
pub const BRIGHT_BLACK: BasicColor = 8;
/// The ANSI bright red color. `9`.
pub const BRIGHT_RED: BasicColor = 9;
/// The ANSI bright green color. `10`.
pub const BRIGHT_GREEN: BasicColor = 10;
/// The ANSI bright yellow color. `11`.
pub const BRIGHT_YELLOW: BasicColor = 11;
/// The ANSI bright blue color. `12`.
pub const BRIGHT_BLUE: BasicColor = 12;
/// The ANSI bright magenta color. `13`.
pub const BRIGHT_MAGENTA: BasicColor = 13;
/// The ANSI bright cyan color. `14`.
pub const BRIGHT_CYAN: BasicColor = 14;
/// The ANSI bright white color. `15`.
pub const BRIGHT_WHITE: BasicColor = 15;

/// ExtendedColor is an ANSI 256 (8-bit) color with a value from 0 to 255.
///
/// Deprecated: use [IndexedColor] instead.
pub type ExtendedColor = IndexedColor;

/// TrueColor is a 24-bit color that can be used in the terminal.
/// This can be used to represent RGB colors. For example, the color red can be
/// represented as `0xff0000`.
///
/// Deprecated: use [RGBColor] instead.
pub type TrueColor = u32;

/// IndexedColor is an 8-bit ANSI color (0-255).
pub type IndexedColor = u8;

/// RGBColor is a 24-bit color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGBColor {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
}
impl RGBColor {
    /// Hex returns the lowercase `#rrggbb` representation of the color.
    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// The xterm-256 palette: 16 basic colors, a 6x6x6 color cube, and grayscale.
pub fn indexed_rgb(c: u8) -> (u8, u8, u8) {
    const PALETTE: [(u8, u8, u8); 16] = [
        (0x00, 0x00, 0x00),
        (0x80, 0x00, 0x00),
        (0x00, 0x80, 0x00),
        (0x80, 0x80, 0x00),
        (0x00, 0x00, 0x80),
        (0x80, 0x00, 0x80),
        (0x00, 0x80, 0x80),
        (0xC0, 0xC0, 0xC0),
        (0x80, 0x80, 0x80),
        (0xFF, 0x00, 0x00),
        (0x00, 0xFF, 0x00),
        (0xFF, 0xFF, 0x00),
        (0x00, 0x00, 0xFF),
        (0xFF, 0x00, 0xFF),
        (0x00, 0xFF, 0xFF),
        (0xFF, 0xFF, 0xFF),
    ];
    match c as usize {
        0..=15 => PALETTE[c as usize],
        16..=231 => {
            let i = c as usize - 16;
            let r = i / 36;
            let g = (i / 6) % 6;
            let b = i % 6;
            let level = |v: usize| -> u8 {
                if v == 0 {
                    0
                } else {
                    (55 + v * 40) as u8
                }
            };
            (level(r), level(g), level(b))
        }
        _ => {
            let gray = 8 + (c as usize - 232) * 10;
            (gray as u8, gray as u8, gray as u8)
        }
    }
}

const Q2C: [i32; 6] = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

fn to6_cube(v: i32) -> usize {
    if v < 48 {
        0
    } else if v < 115 {
        1
    } else {
        ((v - 35) / 40).clamp(0, 5) as usize
    }
}

/// Converts an RGB color to the xterm-256 palette index, mirroring
/// `ansi.Convert256` (tmux cube + HSLuv distance).
pub fn convert_256(r: u8, g: u8, b: u8) -> IndexedColor {
    // The 8-bit inputs are used directly: upstream computes `col.R * 255`
    // from the normalized float color, and `colorful.MakeColor` only applies
    // its 16-bit division to 16-bit channel inputs (e.g. `color.RGBA`), not
    // to pre-normalized colors.
    let r16 = r as f64;
    let g16 = g as f64;
    let b16 = b as f64;

    let qr = to6_cube(r16 as i32);
    let cr = Q2C[qr];
    let qg = to6_cube(g16 as i32);
    let cg = Q2C[qg];
    let qb = to6_cube(b16 as i32);
    let cb = Q2C[qb];

    let ci = 36 * qr + 6 * qg + qb;
    if cr == r16 as i32 && cg == g16 as i32 && cb == b16 as i32 {
        return (16 + ci) as u8;
    }

    let grey_avg = ((r16 + g16 + b16) / 3.0) as i32;
    let grey_idx = if grey_avg > 238 {
        23
    } else {
        ((grey_avg - 3) / 10).clamp(0, 23)
    };
    let grey = 8 + 10 * grey_idx;

    let color_dist = hsluv_distance(
        r16 / 255.0,
        g16 / 255.0,
        b16 / 255.0,
        cr as f64 / 255.0,
        cg as f64 / 255.0,
        cb as f64 / 255.0,
    );
    let gray_dist = hsluv_distance(
        r16 / 255.0,
        g16 / 255.0,
        b16 / 255.0,
        grey as f64 / 255.0,
        grey as f64 / 255.0,
        grey as f64 / 255.0,
    );

    if color_dist <= gray_dist {
        (16 + ci) as u8
    } else {
        (232 + grey_idx) as u8
    }
}

/// The `ansi256To16` conversion table from upstream.
const ANSI256_TO_16: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 4, 4, 4, 12, 12, 2, 6, 4, 4, 12, 12,
    2, 2, 6, 4, 12, 12, 2, 2, 2, 6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14, 1, 5,
    4, 4, 12, 12, 3, 8, 4, 4, 12, 12, 2, 2, 6, 4, 12, 12, 2, 2, 2, 6, 12, 12, 10, 10, 10, 10, 14,
    12, 10, 10, 10, 10, 10, 14, 1, 1, 5, 4, 12, 12, 1, 1, 5, 4, 12, 12, 3, 3, 8, 4, 12, 12, 2, 2,
    2, 6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14, 1, 1, 1, 5, 12, 12, 1, 1, 1, 5,
    12, 12, 1, 1, 1, 5, 12, 12, 3, 3, 3, 7, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14,
    9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 11, 11, 11, 11,
    7, 12, 10, 10, 10, 10, 10, 14, 9, 9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 9, 9,
    9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 11, 11, 11, 11, 11, 15, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 7,
    7, 7, 7, 7, 7, 15, 15, 15, 15, 15, 15,
];

/// Converts an RGB color to a 16-color ANSI color, mirroring `ansi.Convert16`.
pub fn convert_16(r: u8, g: u8, b: u8) -> BasicColor {
    ansi256_to_16(convert_256(r, g, b))
}

/// Converts an 8-bit indexed color to a 16-color ANSI color.
pub fn ansi256_to_16(n: u8) -> BasicColor {
    ANSI256_TO_16[n as usize]
}

// ---------------------------------------------------------------------------
// HSLuv distance (mirrors `go-colorful` DistanceHSLuv)
// ---------------------------------------------------------------------------

const HSLUV_D65: [f64; 3] = [0.95045592705167, 1.0, 1.089057750759878];
const KAPPA: f64 = 903.2962962962963;
const EPSILON: f64 = 0.008_856_451_679_035_631;
const M: [[f64; 3]; 3] = [
    [
        3.2409699419045214,
        -1.5373831775700935,
        -0.498_610_760_293_003_3,
    ],
    [
        -0.969_243_636_280_879_8,
        1.8759675015077207,
        0.041_555_057_407_175_61,
    ],
    [
        0.055_630_079_696_993_61,
        -0.20397695888897657,
        1.0569715142428786,
    ],
];

fn hsluv_distance(r1: f64, g1: f64, b1: f64, r2: f64, g2: f64, b2: f64) -> f64 {
    let (h1, s1, l1) = hsluv(r1, g1, b1);
    let (h2, s2, l2) = hsluv(r2, g2, b2);
    let dh = (h1 - h2) / 100.0;
    let ds = s1 - s2;
    let dl = l1 - l2;
    (dh * dh + ds * ds + dl * dl).sqrt()
}

fn hsluv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (l, c, h) = luv_lch(r, g, b);
    luv_lch_to_hsluv(l, c, h)
}

fn luv_lch(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (x, y, z) = linear_rgb_to_xyz(linearize(r), linearize(g), linearize(b));
    let (l, u, v) = xyz_to_luv_white_ref(x, y, z, HSLUV_D65);
    luv_to_luv_lch(l, u, v)
}

fn linearize(v: f64) -> f64 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_rgb_to_xyz(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    (
        0.412_390_799_265_959_5 * r + 0.35758433938387796 * g + 0.180_480_788_401_834_3 * b,
        0.21263900587151036 * r + 0.715_168_678_767_755_9 * g + 0.072_192_315_360_733_71 * b,
        0.019_330_818_715_591_85 * r + 0.11919477979462599 * g + 0.950_532_152_249_660_6 * b,
    )
}

fn xyz_to_uv(x: f64, y: f64, z: f64) -> (f64, f64) {
    let denom = x + 15.0 * y + 3.0 * z;
    if denom == 0.0 {
        (0.0, 0.0)
    } else {
        (4.0 * x / denom, 9.0 * y / denom)
    }
}

fn xyz_to_luv_white_ref(x: f64, y: f64, z: f64, wref: [f64; 3]) -> (f64, f64, f64) {
    let l = if y / wref[1] <= 6.0 / 29.0 * 6.0 / 29.0 * 6.0 / 29.0 {
        y / wref[1] * (29.0 / 3.0 * 29.0 / 3.0 * 29.0 / 3.0) / 100.0
    } else {
        1.16 * (y / wref[1]).cbrt() - 0.16
    };
    let (ubis, vbis) = xyz_to_uv(x, y, z);
    let (un, vn) = xyz_to_uv(wref[0], wref[1], wref[2]);
    (l, 13.0 * l * (ubis - un), 13.0 * l * (vbis - vn))
}

fn luv_to_luv_lch(l: f64, u: f64, v: f64) -> (f64, f64, f64) {
    let h = if (v - u).abs() > 1e-4 && u.abs() > 1e-4 {
        (57.295_779_513_082_32 * v.atan2(u) + 360.0).rem_euclid(360.0)
    } else {
        0.0
    };
    (l, (u * u + v * v).sqrt(), h)
}

fn luv_lch_to_hsluv(l: f64, c: f64, h: f64) -> (f64, f64, f64) {
    let c = c * 100.0;
    let l = l * 100.0;
    let s = if !(0.00000001..=99.9999999).contains(&l) {
        0.0
    } else {
        let max = max_chroma_for_lh(l, h);
        c / max * 100.0
    };
    (h, (s / 100.0).clamp(0.0, 1.0), (l / 100.0).clamp(0.0, 1.0))
}

fn max_chroma_for_lh(l: f64, h: f64) -> f64 {
    let h_rad = h / 360.0 * std::f64::consts::PI * 2.0;
    let mut min_length = f64::MAX;
    for line in get_bounds(l) {
        let length = length_of_ray_until_intersect(h_rad, line[0], line[1]);
        if length > 0.0 && length < min_length {
            min_length = length;
        }
    }
    min_length
}

fn get_bounds(l: f64) -> [[f64; 2]; 6] {
    let mut ret = [[0.0; 2]; 6];
    let sub1 = (l + 16.0).powi(3) / 1560896.0;
    let sub2 = if sub1 > EPSILON { sub1 } else { l / KAPPA };
    for (i, mrow) in M.iter().enumerate() {
        for k in 0..2 {
            let top1 = (284517.0 * mrow[0] - 94839.0 * mrow[2]) * sub2;
            let top2 = (838422.0 * mrow[2] + 769860.0 * mrow[1] + 731718.0 * mrow[0]) * l * sub2
                - 769860.0 * k as f64 * l;
            let bottom = (632260.0 * mrow[2] - 126452.0 * mrow[1]) * sub2 + 126452.0 * k as f64;
            ret[i * 2 + k][0] = top1 / bottom;
            ret[i * 2 + k][1] = top2 / bottom;
        }
    }
    ret
}

fn length_of_ray_until_intersect(theta: f64, x: f64, y: f64) -> f64 {
    y / (theta.sin() - x * theta.cos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_256() {
        assert_eq!(convert_256(0, 0, 255), 21);
        assert_eq!(convert_256(255, 0, 0), 196);
        assert_eq!(convert_256(0, 255, 0), 46);
    }

    #[test]
    fn test_convert_16() {
        assert_eq!(convert_16(255, 0, 0), 9);
        assert_eq!(ansi256_to_16(240), 8);
    }
}
