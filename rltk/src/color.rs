use super::rex::XpColor;
use std::ops;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(PartialEq, Copy, Clone, Default)]
/// Represents an R/G/B triplet, in the range 0..1 (32-bit float)
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(PartialEq, Copy, Clone, Default)]
/// Represents an H/S/V triplet, in the range 0..1 (32-bit float)
pub struct HSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Error message type when failing to convert a hex code to RGB.
pub enum HtmlColorConversionError {
    InvalidStringLength,
    MissingHash,
    InvalidCharacter,
}

// Implement operator overloading

/// Support adding a float to a color. The result is clamped via the constructor.
impl ops::Add<f32> for RGB {
    type Output = RGB;
    fn add(mut self, rhs: f32) -> RGB {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self
    }
}

/// Support adding an RGB to a color. The result is clamped via the constructor.
impl ops::Add<RGB> for RGB {
    type Output = RGB;
    fn add(mut self, rhs: RGB) -> RGB {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self
    }
}

/// Support subtracting a float from a color. The result is clamped via the constructor.
impl ops::Sub<f32> for RGB {
    type Output = RGB;
    fn sub(mut self, rhs: f32) -> RGB {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self
    }
}

/// Support subtracting an RGB from a color. The result is clamped via the constructor.
impl ops::Sub<RGB> for RGB {
    type Output = RGB;
    fn sub(mut self, rhs: RGB) -> RGB {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self
    }
}

/// Support multiplying a color by a float. The result is clamped via the constructor.
impl ops::Mul<f32> for RGB {
    type Output = RGB;
    fn mul(mut self, rhs: f32) -> RGB {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self
    }
}

/// Support multiplying a color by another color. The result is clamped via the constructor.
impl ops::Mul<RGB> for RGB {
    type Output = RGB;
    fn mul(mut self, rhs: RGB) -> RGB {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self
    }
}

impl RGB {
    /// Constructs a new, zeroed (black) RGB triplet.
    pub fn new() -> RGB {
        RGB {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Constructs a new RGB color, from 3 32-bit floats in the range 0..1
    pub fn from_f32(r: f32, g: f32, b: f32) -> RGB {
        let r_clamped = f32::min(1.0, f32::max(0.0, r));
        let g_clamped = f32::min(1.0, f32::max(0.0, g));
        let b_clamped = f32::min(1.0, f32::max(0.0, b));
        RGB {
            r: r_clamped,
            g: g_clamped,
            b: b_clamped,
        }
    }

    /// Constructs a new RGB color, from 3 bytes in the range 0..255
    pub fn from_u8(r: u8, g: u8, b: u8) -> RGB {
        RGB {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
        }
    }

    /// Construct an RGB color from a tuple of u8, or a named constant
    pub fn named(col: (u8, u8, u8)) -> RGB {
        RGB::from_u8(col.0, col.1, col.2)
    }

    /// Constructs from an HTML color code (e.g. "#eeffee")
    pub fn from_hex<S: AsRef<str>>(code: S) -> Result<RGB, HtmlColorConversionError> {
        let mut full_code = code.as_ref().chars();

        if let Some(hash) = full_code.next() {
            if hash != '#' {
                return Err(HtmlColorConversionError::MissingHash);
            }
        } else {
            return Err(HtmlColorConversionError::InvalidStringLength);
        }

        let red1 = match full_code.next() {
            Some(red) => match red.to_digit(16) {
                Some(red) => red * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let red2 = match full_code.next() {
            Some(red) => match red.to_digit(16) {
                Some(red) => red,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        let green1 = match full_code.next() {
            Some(green) => match green.to_digit(16) {
                Some(green) => green * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let green2 = match full_code.next() {
            Some(green) => match green.to_digit(16) {
                Some(green) => green,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        let blue1 = match full_code.next() {
            Some(blue) => match blue.to_digit(16) {
                Some(blue) => blue * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let blue2 = match full_code.next() {
            Some(blue) => match blue.to_digit(16) {
                Some(blue) => blue,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        if full_code.next().is_some() {
            return Err(HtmlColorConversionError::InvalidStringLength);
        }

        Ok(RGB {
            r: (red1 + red2) as f32 / 255.0,
            g: (green1 + green2) as f32 / 255.0,
            b: (blue1 + blue2) as f32 / 255.0,
        })
    }

    /// Converts an xp file color component to an RGB
    pub fn from_xp(col: XpColor) -> RGB {
        RGB::from_u8(col.r, col.g, col.b)
    }

    /// Converts an RGB to an xp file color component
    pub fn to_xp(&self) -> XpColor {
        XpColor::new(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    /// Converts an RGB triple to an HSV triple.
    #[allow(clippy::many_single_char_names)]
    pub fn to_hsv(&self) -> HSV {
        let r = self.r;
        let g = self.g;
        let b = self.b;

        let max = f32::max(f32::max(r, g), b);
        let min = f32::min(f32::min(r, g), b);

        let mut h: f32 = max;
        let s: f32;
        let v: f32 = max;

        let d = max - min;
        if max == 0.0 {
            s = 0.0;
        } else {
            s = d / max;
        }

        if (max - min).abs() < std::f32::EPSILON {
            h = 0.0; // Achromatic
        } else {
            if (max - r).abs() < std::f32::EPSILON {
                if g < b {
                    h = (g - b) / d + 6.0;
                } else {
                    h = (g - b) / d;
                }
            } else if (max - g).abs() < std::f32::EPSILON {
                h = (b - r) / d + 2.0;
            } else if (max - b).abs() < std::f32::EPSILON {
                h = (r - g) / d + 4.0;
            }

            h /= 6.0;
        }

        HSV::from_f32(h, s, v)
    }

    /// Applies a quick grayscale conversion to the color
    pub fn to_greyscale(&self) -> RGB {
        let linear = (self.r * 0.2126) + (self.g * 0.7152) + (self.b * 0.0722);
        RGB::from_f32(linear, linear, linear)
    }

    /// Applies a lengthier desaturate (via HSV) to the color
    pub fn desaturate(&self) -> RGB {
        let mut hsv = self.to_hsv();
        hsv.s = 0.0;
        hsv.to_rgb()
    }

    /// Lerps by a specified percentage (from 0 to 1) between this color and another
    pub fn lerp(&self, color: RGB, percent: f32) -> RGB {
        let range = (color.r - self.r, color.g - self.g, color.b - self.b);
        RGB {
            r: self.r + range.0 * percent,
            g: self.g + range.1 * percent,
            b: self.b + range.2 * percent,
        }
    }
}

impl HSV {
    /// Constructs a new, zeroed (black) HSV triplet.
    pub fn new() -> HSV {
        HSV {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        }
    }

    /// Constructs a new HSV color, from 3 32-bit floats
    pub fn from_f32(h: f32, s: f32, v: f32) -> HSV {
        HSV { h, s, v }
    }

    /// Converts an HSV triple to an RGB triple
    #[allow(clippy::many_single_char_names)] // I like my short names for this one
    pub fn to_rgb(&self) -> RGB {
        let h = self.h;
        let s = self.s;
        let v = self.v;

        let mut r: f32 = 0.0;
        let mut g: f32 = 0.0;
        let mut b: f32 = 0.0;

        let i = f32::floor(h * 6.0) as i32;
        let f = h * 6.0 - i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match i % 6 {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }
            _ => {}
        }

        RGB::from_f32(r, g, b)
    }
}

// Named Colors (derived from X11 rgb.txt, which is also the source of HTML/W3C/SVG names)
pub const SNOW: (u8, u8, u8) = (255, 250, 250);
pub const GHOST_WHITE: (u8, u8, u8) = (248, 248, 255);
pub const GHOSTWHITE: (u8, u8, u8) = (248, 248, 255);
pub const WHITE_SMOKE: (u8, u8, u8) = (245, 245, 245);
pub const WHITESMOKE: (u8, u8, u8) = (245, 245, 245);
pub const GAINSBORO: (u8, u8, u8) = (220, 220, 220);
pub const FLORAL_WHITE: (u8, u8, u8) = (255, 250, 240);
pub const FLORALWHITE: (u8, u8, u8) = (255, 250, 240);
pub const OLD_LACE: (u8, u8, u8) = (253, 245, 230);
pub const OLDLACE: (u8, u8, u8) = (253, 245, 230);
pub const LINEN: (u8, u8, u8) = (250, 240, 230);
pub const ANTIQUE_WHITE: (u8, u8, u8) = (250, 235, 215);
pub const ANTIQUEWHITE: (u8, u8, u8) = (250, 235, 215);
pub const PAPAYA_WHIP: (u8, u8, u8) = (255, 239, 213);
pub const PAPAYAWHIP: (u8, u8, u8) = (255, 239, 213);
pub const BLANCHED_ALMOND: (u8, u8, u8) = (255, 235, 205);
pub const BLANCHEDALMOND: (u8, u8, u8) = (255, 235, 205);
pub const BISQUE: (u8, u8, u8) = (255, 228, 196);
pub const PEACH_PUFF: (u8, u8, u8) = (255, 218, 185);
pub const PEACHPUFF: (u8, u8, u8) = (255, 218, 185);
pub const NAVAJO_WHITE: (u8, u8, u8) = (255, 222, 173);
pub const NAVAJOWHITE: (u8, u8, u8) = (255, 222, 173);
pub const MOCCASIN: (u8, u8, u8) = (255, 228, 181);
pub const CORNSILK: (u8, u8, u8) = (255, 248, 220);
pub const IVORY: (u8, u8, u8) = (255, 255, 240);
pub const LEMON_CHIFFON: (u8, u8, u8) = (255, 250, 205);
pub const LEMONCHIFFON: (u8, u8, u8) = (255, 250, 205);
pub const SEASHELL: (u8, u8, u8) = (255, 245, 238);
pub const HONEYDEW: (u8, u8, u8) = (240, 255, 240);
pub const MINT_CREAM: (u8, u8, u8) = (245, 255, 250);
pub const MINTCREAM: (u8, u8, u8) = (245, 255, 250);
pub const AZURE: (u8, u8, u8) = (240, 255, 255);
pub const ALICE_BLUE: (u8, u8, u8) = (240, 248, 255);
pub const ALICEBLUE: (u8, u8, u8) = (240, 248, 255);
pub const LAVENDER: (u8, u8, u8) = (230, 230, 250);
pub const LAVENDER_BLUSH: (u8, u8, u8) = (255, 240, 245);
pub const LAVENDERBLUSH: (u8, u8, u8) = (255, 240, 245);
pub const MISTY_ROSE: (u8, u8, u8) = (255, 228, 225);
pub const MISTYROSE: (u8, u8, u8) = (255, 228, 225);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const BLACK: (u8, u8, u8) = (0, 0, 0);
pub const DARK_SLATE: (u8, u8, u8) = (47, 79, 79);
pub const DARKSLATEGRAY: (u8, u8, u8) = (47, 79, 79);
pub const DARKSLATEGREY: (u8, u8, u8) = (47, 79, 79);
pub const DIM_GRAY: (u8, u8, u8) = (105, 105, 105);
pub const DIMGRAY: (u8, u8, u8) = (105, 105, 105);
pub const DIM_GREY: (u8, u8, u8) = (105, 105, 105);
pub const DIMGREY: (u8, u8, u8) = (105, 105, 105);
pub const SLATE_GRAY: (u8, u8, u8) = (112, 128, 144);
pub const SLATEGRAY: (u8, u8, u8) = (112, 128, 144);
pub const SLATE_GREY: (u8, u8, u8) = (112, 128, 144);
pub const SLATEGREY: (u8, u8, u8) = (112, 128, 144);
pub const LIGHT_SLATE: (u8, u8, u8) = (119, 136, 153);
pub const LIGHTSLATEGRAY: (u8, u8, u8) = (119, 136, 153);
pub const LIGHTSLATEGREY: (u8, u8, u8) = (119, 136, 153);
pub const GRAY: (u8, u8, u8) = (190, 190, 190);
pub const GREY: (u8, u8, u8) = (190, 190, 190);
pub const X11_GRAY: (u8, u8, u8) = (190, 190, 190);
pub const X11GRAY: (u8, u8, u8) = (190, 190, 190);
pub const X11_GREY: (u8, u8, u8) = (190, 190, 190);
pub const X11GREY: (u8, u8, u8) = (190, 190, 190);
pub const WEB_GRAY: (u8, u8, u8) = (128, 128, 128);
pub const WEBGRAY: (u8, u8, u8) = (128, 128, 128);
pub const WEB_GREY: (u8, u8, u8) = (128, 128, 128);
pub const WEBGREY: (u8, u8, u8) = (128, 128, 128);
pub const LIGHT_GREY: (u8, u8, u8) = (211, 211, 211);
pub const LIGHTGREY: (u8, u8, u8) = (211, 211, 211);
pub const LIGHT_GRAY: (u8, u8, u8) = (211, 211, 211);
pub const LIGHTGRAY: (u8, u8, u8) = (211, 211, 211);
pub const MIDNIGHT_BLUE: (u8, u8, u8) = (25, 25, 112);
pub const MIDNIGHTBLUE: (u8, u8, u8) = (25, 25, 112);
pub const NAVY: (u8, u8, u8) = (0, 0, 128);
pub const NAVY_BLUE: (u8, u8, u8) = (0, 0, 128);
pub const NAVYBLUE: (u8, u8, u8) = (0, 0, 128);
pub const CORNFLOWER_BLUE: (u8, u8, u8) = (100, 149, 237);
pub const CORNFLOWERBLUE: (u8, u8, u8) = (100, 149, 237);
pub const DARKSLATEBLUE: (u8, u8, u8) = (72, 61, 139);
pub const SLATE_BLUE: (u8, u8, u8) = (106, 90, 205);
pub const SLATEBLUE: (u8, u8, u8) = (106, 90, 205);
pub const MEDIUM_SLATE: (u8, u8, u8) = (123, 104, 238);
pub const MEDIUMSLATEBLUE: (u8, u8, u8) = (123, 104, 238);
pub const LIGHTSLATEBLUE: (u8, u8, u8) = (132, 112, 255);
pub const MEDIUM_BLUE: (u8, u8, u8) = (0, 0, 205);
pub const MEDIUMBLUE: (u8, u8, u8) = (0, 0, 205);
pub const ROYAL_BLUE: (u8, u8, u8) = (65, 105, 225);
pub const ROYALBLUE: (u8, u8, u8) = (65, 105, 225);
pub const BLUE: (u8, u8, u8) = (0, 0, 255);
pub const DODGER_BLUE: (u8, u8, u8) = (30, 144, 255);
pub const DODGERBLUE: (u8, u8, u8) = (30, 144, 255);
pub const DEEP_SKY: (u8, u8, u8) = (0, 191, 255);
pub const DEEPSKYBLUE: (u8, u8, u8) = (0, 191, 255);
pub const SKY_BLUE: (u8, u8, u8) = (135, 206, 235);
pub const SKYBLUE: (u8, u8, u8) = (135, 206, 235);
pub const LIGHT_SKY: (u8, u8, u8) = (135, 206, 250);
pub const LIGHTSKYBLUE: (u8, u8, u8) = (135, 206, 250);
pub const STEEL_BLUE: (u8, u8, u8) = (70, 130, 180);
pub const STEELBLUE: (u8, u8, u8) = (70, 130, 180);
pub const LIGHT_STEEL: (u8, u8, u8) = (176, 196, 222);
pub const LIGHTSTEELBLUE: (u8, u8, u8) = (176, 196, 222);
pub const LIGHT_BLUE: (u8, u8, u8) = (173, 216, 230);
pub const LIGHTBLUE: (u8, u8, u8) = (173, 216, 230);
pub const POWDER_BLUE: (u8, u8, u8) = (176, 224, 230);
pub const POWDERBLUE: (u8, u8, u8) = (176, 224, 230);
pub const PALE_TURQUOISE: (u8, u8, u8) = (175, 238, 238);
pub const PALETURQUOISE: (u8, u8, u8) = (175, 238, 238);
pub const DARK_TURQUOISE: (u8, u8, u8) = (0, 206, 209);
pub const DARKTURQUOISE: (u8, u8, u8) = (0, 206, 209);
pub const MEDIUM_TURQUOISE: (u8, u8, u8) = (72, 209, 204);
pub const MEDIUMTURQUOISE: (u8, u8, u8) = (72, 209, 204);
pub const TURQUOISE: (u8, u8, u8) = (64, 224, 208);
pub const CYAN: (u8, u8, u8) = (0, 255, 255);
pub const AQUA: (u8, u8, u8) = (0, 255, 255);
pub const LIGHT_CYAN: (u8, u8, u8) = (224, 255, 255);
pub const LIGHTCYAN: (u8, u8, u8) = (224, 255, 255);
pub const CADET_BLUE: (u8, u8, u8) = (95, 158, 160);
pub const CADETBLUE: (u8, u8, u8) = (95, 158, 160);
pub const MEDIUM_AQUAMARINE: (u8, u8, u8) = (102, 205, 170);
pub const MEDIUMAQUAMARINE: (u8, u8, u8) = (102, 205, 170);
pub const AQUAMARINE: (u8, u8, u8) = (127, 255, 212);
pub const DARK_GREEN: (u8, u8, u8) = (0, 100, 0);
pub const DARKGREEN: (u8, u8, u8) = (0, 100, 0);
pub const DARK_OLIVE: (u8, u8, u8) = (85, 107, 47);
pub const DARKOLIVEGREEN: (u8, u8, u8) = (85, 107, 47);
pub const DARK_SEA: (u8, u8, u8) = (143, 188, 143);
pub const DARKSEAGREEN: (u8, u8, u8) = (143, 188, 143);
pub const SEA_GREEN: (u8, u8, u8) = (46, 139, 87);
pub const SEAGREEN: (u8, u8, u8) = (46, 139, 87);
pub const MEDIUM_SEA: (u8, u8, u8) = (60, 179, 113);
pub const MEDIUMSEAGREEN: (u8, u8, u8) = (60, 179, 113);
pub const LIGHT_SEA: (u8, u8, u8) = (32, 178, 170);
pub const LIGHTSEAGREEN: (u8, u8, u8) = (32, 178, 170);
pub const PALE_GREEN: (u8, u8, u8) = (152, 251, 152);
pub const PALEGREEN: (u8, u8, u8) = (152, 251, 152);
pub const SPRING_GREEN: (u8, u8, u8) = (0, 255, 127);
pub const SPRINGGREEN: (u8, u8, u8) = (0, 255, 127);
pub const LAWN_GREEN: (u8, u8, u8) = (124, 252, 0);
pub const LAWNGREEN: (u8, u8, u8) = (124, 252, 0);
pub const GREEN: (u8, u8, u8) = (0, 255, 0);
pub const LIME: (u8, u8, u8) = (0, 255, 0);
pub const X11_GREEN: (u8, u8, u8) = (0, 255, 0);
pub const X11GREEN: (u8, u8, u8) = (0, 255, 0);
pub const WEB_GREEN: (u8, u8, u8) = (0, 128, 0);
pub const WEBGREEN: (u8, u8, u8) = (0, 128, 0);
pub const CHARTREUSE: (u8, u8, u8) = (127, 255, 0);
pub const MEDIUM_SPRING: (u8, u8, u8) = (0, 250, 154);
pub const MEDIUMSPRINGGREEN: (u8, u8, u8) = (0, 250, 154);
pub const GREEN_YELLOW: (u8, u8, u8) = (173, 255, 47);
pub const GREENYELLOW: (u8, u8, u8) = (173, 255, 47);
pub const LIME_GREEN: (u8, u8, u8) = (50, 205, 50);
pub const LIMEGREEN: (u8, u8, u8) = (50, 205, 50);
pub const YELLOW_GREEN: (u8, u8, u8) = (154, 205, 50);
pub const YELLOWGREEN: (u8, u8, u8) = (154, 205, 50);
pub const FOREST_GREEN: (u8, u8, u8) = (34, 139, 34);
pub const FORESTGREEN: (u8, u8, u8) = (34, 139, 34);
pub const OLIVE_DRAB: (u8, u8, u8) = (107, 142, 35);
pub const OLIVEDRAB: (u8, u8, u8) = (107, 142, 35);
pub const DARK_KHAKI: (u8, u8, u8) = (189, 183, 107);
pub const DARKKHAKI: (u8, u8, u8) = (189, 183, 107);
pub const KHAKI: (u8, u8, u8) = (240, 230, 140);
pub const PALE_GOLDENROD: (u8, u8, u8) = (238, 232, 170);
pub const PALEGOLDENROD: (u8, u8, u8) = (238, 232, 170);
pub const LIGHT_GOLDENROD: (u8, u8, u8) = (250, 250, 210);
pub const LIGHTGOLDENRODYELLOW: (u8, u8, u8) = (250, 250, 210);
pub const LIGHT_YELLOW: (u8, u8, u8) = (255, 255, 224);
pub const LIGHTYELLOW: (u8, u8, u8) = (255, 255, 224);
pub const YELLOW: (u8, u8, u8) = (255, 255, 0);
pub const GOLD: (u8, u8, u8) = (255, 215, 0);
pub const LIGHTGOLDENROD: (u8, u8, u8) = (238, 221, 130);
pub const GOLDENROD: (u8, u8, u8) = (218, 165, 32);
pub const DARK_GOLDENROD: (u8, u8, u8) = (184, 134, 11);
pub const DARKGOLDENROD: (u8, u8, u8) = (184, 134, 11);
pub const ROSY_BROWN: (u8, u8, u8) = (188, 143, 143);
pub const ROSYBROWN: (u8, u8, u8) = (188, 143, 143);
pub const INDIAN_RED: (u8, u8, u8) = (205, 92, 92);
pub const INDIANRED: (u8, u8, u8) = (205, 92, 92);
pub const SADDLE_BROWN: (u8, u8, u8) = (139, 69, 19);
pub const SADDLEBROWN: (u8, u8, u8) = (139, 69, 19);
pub const SIENNA: (u8, u8, u8) = (160, 82, 45);
pub const PERU: (u8, u8, u8) = (205, 133, 63);
pub const BURLYWOOD: (u8, u8, u8) = (222, 184, 135);
pub const BEIGE: (u8, u8, u8) = (245, 245, 220);
pub const WHEAT: (u8, u8, u8) = (245, 222, 179);
pub const SANDY_BROWN: (u8, u8, u8) = (244, 164, 96);
pub const SANDYBROWN: (u8, u8, u8) = (244, 164, 96);
pub const TAN: (u8, u8, u8) = (210, 180, 140);
pub const CHOCOLATE: (u8, u8, u8) = (210, 105, 30);
pub const FIREBRICK_34: (u8, u8, u8) = (178, 34, 34);
pub const BROWN_42: (u8, u8, u8) = (165, 42, 42);
pub const DARK_SALMON: (u8, u8, u8) = (233, 150, 122);
pub const DARKSALMON: (u8, u8, u8) = (233, 150, 122);
pub const SALMON: (u8, u8, u8) = (250, 128, 114);
pub const LIGHT_SALMON: (u8, u8, u8) = (255, 160, 122);
pub const LIGHTSALMON: (u8, u8, u8) = (255, 160, 122);
pub const ORANGE: (u8, u8, u8) = (255, 165, 0);
pub const DARK_ORANGE: (u8, u8, u8) = (255, 140, 0);
pub const DARKORANGE: (u8, u8, u8) = (255, 140, 0);
pub const CORAL: (u8, u8, u8) = (255, 127, 80);
pub const LIGHT_CORAL: (u8, u8, u8) = (240, 128, 128);
pub const LIGHTCORAL: (u8, u8, u8) = (240, 128, 128);
pub const TOMATO: (u8, u8, u8) = (255, 99, 71);
pub const ORANGE_RED: (u8, u8, u8) = (255, 69, 0);
pub const ORANGERED: (u8, u8, u8) = (255, 69, 0);
pub const RED: (u8, u8, u8) = (255, 0, 0);
pub const HOT_PINK: (u8, u8, u8) = (255, 105, 180);
pub const HOTPINK: (u8, u8, u8) = (255, 105, 180);
pub const DEEP_PINK: (u8, u8, u8) = (255, 20, 147);
pub const DEEPPINK: (u8, u8, u8) = (255, 20, 147);
pub const PINK: (u8, u8, u8) = (255, 192, 203);
pub const LIGHT_PINK: (u8, u8, u8) = (255, 182, 193);
pub const LIGHTPINK: (u8, u8, u8) = (255, 182, 193);
pub const PALE_VIOLET: (u8, u8, u8) = (219, 112, 147);
pub const PALEVIOLETRED: (u8, u8, u8) = (219, 112, 147);
pub const MAROON: (u8, u8, u8) = (176, 48, 96);
pub const X11_MAROON: (u8, u8, u8) = (176, 48, 96);
pub const X11MAROON: (u8, u8, u8) = (176, 48, 96);
pub const WEB_MAROON: (u8, u8, u8) = (128, 0, 0);
pub const WEBMAROON: (u8, u8, u8) = (128, 0, 0);
pub const MEDIUM_VIOLET: (u8, u8, u8) = (199, 21, 133);
pub const MEDIUMVIOLETRED: (u8, u8, u8) = (199, 21, 133);
pub const VIOLET_RED: (u8, u8, u8) = (208, 32, 144);
pub const VIOLETRED: (u8, u8, u8) = (208, 32, 144);
pub const MAGENTA: (u8, u8, u8) = (255, 0, 255);
pub const FUCHSIA: (u8, u8, u8) = (255, 0, 255);
pub const VIOLET: (u8, u8, u8) = (238, 130, 238);
pub const PLUM: (u8, u8, u8) = (221, 160, 221);
pub const ORCHID: (u8, u8, u8) = (218, 112, 214);
pub const MEDIUM_ORCHID: (u8, u8, u8) = (186, 85, 211);
pub const MEDIUMORCHID: (u8, u8, u8) = (186, 85, 211);
pub const DARK_ORCHID: (u8, u8, u8) = (153, 50, 204);
pub const DARKORCHID: (u8, u8, u8) = (153, 50, 204);
pub const DARK_VIOLET: (u8, u8, u8) = (148, 0, 211);
pub const DARKVIOLET: (u8, u8, u8) = (148, 0, 211);
pub const BLUE_VIOLET: (u8, u8, u8) = (138, 43, 226);
pub const BLUEVIOLET: (u8, u8, u8) = (138, 43, 226);
pub const PURPLE: (u8, u8, u8) = (160, 32, 240);
pub const X11_PURPLE: (u8, u8, u8) = (160, 32, 240);
pub const X11PURPLE: (u8, u8, u8) = (160, 32, 240);
pub const WEB_PURPLE: (u8, u8, u8) = (128, 0, 128);
pub const WEBPURPLE: (u8, u8, u8) = (128, 0, 128);
pub const MEDIUM_PURPLE: (u8, u8, u8) = (147, 112, 219);
pub const MEDIUMPURPLE: (u8, u8, u8) = (147, 112, 219);
pub const THISTLE: (u8, u8, u8) = (216, 191, 216);
pub const SNOW1: (u8, u8, u8) = (255, 250, 250);
pub const SNOW2: (u8, u8, u8) = (238, 233, 233);
pub const SNOW3: (u8, u8, u8) = (205, 201, 201);
pub const SNOW4: (u8, u8, u8) = (139, 137, 137);
pub const SEASHELL1: (u8, u8, u8) = (255, 245, 238);
pub const SEASHELL2: (u8, u8, u8) = (238, 229, 222);
pub const SEASHELL3: (u8, u8, u8) = (205, 197, 191);
pub const SEASHELL4: (u8, u8, u8) = (139, 134, 130);
pub const ANTIQUEWHITE1: (u8, u8, u8) = (255, 239, 219);
pub const ANTIQUEWHITE2: (u8, u8, u8) = (238, 223, 204);
pub const ANTIQUEWHITE3: (u8, u8, u8) = (205, 192, 176);
pub const ANTIQUEWHITE4: (u8, u8, u8) = (139, 131, 120);
pub const BISQUE1: (u8, u8, u8) = (255, 228, 196);
pub const BISQUE2: (u8, u8, u8) = (238, 213, 183);
pub const BISQUE3: (u8, u8, u8) = (205, 183, 158);
pub const BISQUE4: (u8, u8, u8) = (139, 125, 107);
pub const PEACHPUFF1: (u8, u8, u8) = (255, 218, 185);
pub const PEACHPUFF2: (u8, u8, u8) = (238, 203, 173);
pub const PEACHPUFF3: (u8, u8, u8) = (205, 175, 149);
pub const PEACHPUFF4: (u8, u8, u8) = (139, 119, 101);
pub const NAVAJOWHITE1: (u8, u8, u8) = (255, 222, 173);
pub const NAVAJOWHITE2: (u8, u8, u8) = (238, 207, 161);
pub const NAVAJOWHITE3: (u8, u8, u8) = (205, 179, 139);
pub const NAVAJOWHITE4: (u8, u8, u8) = (139, 121, 94);
pub const LEMONCHIFFON1: (u8, u8, u8) = (255, 250, 205);
pub const LEMONCHIFFON2: (u8, u8, u8) = (238, 233, 191);
pub const LEMONCHIFFON3: (u8, u8, u8) = (205, 201, 165);
pub const LEMONCHIFFON4: (u8, u8, u8) = (139, 137, 112);
pub const CORNSILK1: (u8, u8, u8) = (255, 248, 220);
pub const CORNSILK2: (u8, u8, u8) = (238, 232, 205);
pub const CORNSILK3: (u8, u8, u8) = (205, 200, 177);
pub const CORNSILK4: (u8, u8, u8) = (139, 136, 120);
pub const IVORY1: (u8, u8, u8) = (255, 255, 240);
pub const IVORY2: (u8, u8, u8) = (238, 238, 224);
pub const IVORY3: (u8, u8, u8) = (205, 205, 193);
pub const IVORY4: (u8, u8, u8) = (139, 139, 131);
pub const HONEYDEW1: (u8, u8, u8) = (240, 255, 240);
pub const HONEYDEW2: (u8, u8, u8) = (224, 238, 224);
pub const HONEYDEW3: (u8, u8, u8) = (193, 205, 193);
pub const HONEYDEW4: (u8, u8, u8) = (131, 139, 131);
pub const LAVENDERBLUSH1: (u8, u8, u8) = (255, 240, 245);
pub const LAVENDERBLUSH2: (u8, u8, u8) = (238, 224, 229);
pub const LAVENDERBLUSH3: (u8, u8, u8) = (205, 193, 197);
pub const LAVENDERBLUSH4: (u8, u8, u8) = (139, 131, 134);
pub const MISTYROSE1: (u8, u8, u8) = (255, 228, 225);
pub const MISTYROSE2: (u8, u8, u8) = (238, 213, 210);
pub const MISTYROSE3: (u8, u8, u8) = (205, 183, 181);
pub const MISTYROSE4: (u8, u8, u8) = (139, 125, 123);
pub const AZURE1: (u8, u8, u8) = (240, 255, 255);
pub const AZURE2: (u8, u8, u8) = (224, 238, 238);
pub const AZURE3: (u8, u8, u8) = (193, 205, 205);
pub const AZURE4: (u8, u8, u8) = (131, 139, 139);
pub const SLATEBLUE1: (u8, u8, u8) = (131, 111, 255);
pub const SLATEBLUE2: (u8, u8, u8) = (122, 103, 238);
pub const SLATEBLUE3: (u8, u8, u8) = (105, 89, 205);
pub const SLATEBLUE4: (u8, u8, u8) = (71, 60, 139);
pub const ROYALBLUE1: (u8, u8, u8) = (72, 118, 255);
pub const ROYALBLUE2: (u8, u8, u8) = (67, 110, 238);
pub const ROYALBLUE3: (u8, u8, u8) = (58, 95, 205);
pub const ROYALBLUE4: (u8, u8, u8) = (39, 64, 139);
pub const BLUE1: (u8, u8, u8) = (0, 0, 255);
pub const BLUE2: (u8, u8, u8) = (0, 0, 238);
pub const BLUE3: (u8, u8, u8) = (0, 0, 205);
pub const BLUE4: (u8, u8, u8) = (0, 0, 139);
pub const DODGERBLUE1: (u8, u8, u8) = (30, 144, 255);
pub const DODGERBLUE2: (u8, u8, u8) = (28, 134, 238);
pub const DODGERBLUE3: (u8, u8, u8) = (24, 116, 205);
pub const DODGERBLUE4: (u8, u8, u8) = (16, 78, 139);
pub const STEELBLUE1: (u8, u8, u8) = (99, 184, 255);
pub const STEELBLUE2: (u8, u8, u8) = (92, 172, 238);
pub const STEELBLUE3: (u8, u8, u8) = (79, 148, 205);
pub const STEELBLUE4: (u8, u8, u8) = (54, 100, 139);
pub const DEEPSKYBLUE1: (u8, u8, u8) = (0, 191, 255);
pub const DEEPSKYBLUE2: (u8, u8, u8) = (0, 178, 238);
pub const DEEPSKYBLUE3: (u8, u8, u8) = (0, 154, 205);
pub const DEEPSKYBLUE4: (u8, u8, u8) = (0, 104, 139);
pub const SKYBLUE1: (u8, u8, u8) = (135, 206, 255);
pub const SKYBLUE2: (u8, u8, u8) = (126, 192, 238);
pub const SKYBLUE3: (u8, u8, u8) = (108, 166, 205);
pub const SKYBLUE4: (u8, u8, u8) = (74, 112, 139);
pub const LIGHTSKYBLUE1: (u8, u8, u8) = (176, 226, 255);
pub const LIGHTSKYBLUE2: (u8, u8, u8) = (164, 211, 238);
pub const LIGHTSKYBLUE3: (u8, u8, u8) = (141, 182, 205);
pub const LIGHTSKYBLUE4: (u8, u8, u8) = (96, 123, 139);
pub const SLATEGRAY1: (u8, u8, u8) = (198, 226, 255);
pub const SLATEGRAY2: (u8, u8, u8) = (185, 211, 238);
pub const SLATEGRAY3: (u8, u8, u8) = (159, 182, 205);
pub const SLATEGRAY4: (u8, u8, u8) = (108, 123, 139);
pub const LIGHTSTEELBLUE1: (u8, u8, u8) = (202, 225, 255);
pub const LIGHTSTEELBLUE2: (u8, u8, u8) = (188, 210, 238);
pub const LIGHTSTEELBLUE3: (u8, u8, u8) = (162, 181, 205);
pub const LIGHTSTEELBLUE4: (u8, u8, u8) = (110, 123, 139);
pub const LIGHTBLUE1: (u8, u8, u8) = (191, 239, 255);
pub const LIGHTBLUE2: (u8, u8, u8) = (178, 223, 238);
pub const LIGHTBLUE3: (u8, u8, u8) = (154, 192, 205);
pub const LIGHTBLUE4: (u8, u8, u8) = (104, 131, 139);
pub const LIGHTCYAN1: (u8, u8, u8) = (224, 255, 255);
pub const LIGHTCYAN2: (u8, u8, u8) = (209, 238, 238);
pub const LIGHTCYAN3: (u8, u8, u8) = (180, 205, 205);
pub const LIGHTCYAN4: (u8, u8, u8) = (122, 139, 139);
pub const PALETURQUOISE1: (u8, u8, u8) = (187, 255, 255);
pub const PALETURQUOISE2: (u8, u8, u8) = (174, 238, 238);
pub const PALETURQUOISE3: (u8, u8, u8) = (150, 205, 205);
pub const PALETURQUOISE4: (u8, u8, u8) = (102, 139, 139);
pub const CADETBLUE1: (u8, u8, u8) = (152, 245, 255);
pub const CADETBLUE2: (u8, u8, u8) = (142, 229, 238);
pub const CADETBLUE3: (u8, u8, u8) = (122, 197, 205);
pub const CADETBLUE4: (u8, u8, u8) = (83, 134, 139);
pub const TURQUOISE1: (u8, u8, u8) = (0, 245, 255);
pub const TURQUOISE2: (u8, u8, u8) = (0, 229, 238);
pub const TURQUOISE3: (u8, u8, u8) = (0, 197, 205);
pub const TURQUOISE4: (u8, u8, u8) = (0, 134, 139);
pub const CYAN1: (u8, u8, u8) = (0, 255, 255);
pub const CYAN2: (u8, u8, u8) = (0, 238, 238);
pub const CYAN3: (u8, u8, u8) = (0, 205, 205);
pub const CYAN4: (u8, u8, u8) = (0, 139, 139);
pub const DARKSLATEGRAY1: (u8, u8, u8) = (151, 255, 255);
pub const DARKSLATEGRAY2: (u8, u8, u8) = (141, 238, 238);
pub const DARKSLATEGRAY3: (u8, u8, u8) = (121, 205, 205);
pub const DARKSLATEGRAY4: (u8, u8, u8) = (82, 139, 139);
pub const AQUAMARINE1: (u8, u8, u8) = (127, 255, 212);
pub const AQUAMARINE2: (u8, u8, u8) = (118, 238, 198);
pub const AQUAMARINE3: (u8, u8, u8) = (102, 205, 170);
pub const AQUAMARINE4: (u8, u8, u8) = (69, 139, 116);
pub const DARKSEAGREEN1: (u8, u8, u8) = (193, 255, 193);
pub const DARKSEAGREEN2: (u8, u8, u8) = (180, 238, 180);
pub const DARKSEAGREEN3: (u8, u8, u8) = (155, 205, 155);
pub const DARKSEAGREEN4: (u8, u8, u8) = (105, 139, 105);
pub const SEAGREEN1: (u8, u8, u8) = (84, 255, 159);
pub const SEAGREEN2: (u8, u8, u8) = (78, 238, 148);
pub const SEAGREEN3: (u8, u8, u8) = (67, 205, 128);
pub const SEAGREEN4: (u8, u8, u8) = (46, 139, 87);
pub const PALEGREEN1: (u8, u8, u8) = (154, 255, 154);
pub const PALEGREEN2: (u8, u8, u8) = (144, 238, 144);
pub const PALEGREEN3: (u8, u8, u8) = (124, 205, 124);
pub const PALEGREEN4: (u8, u8, u8) = (84, 139, 84);
pub const SPRINGGREEN1: (u8, u8, u8) = (0, 255, 127);
pub const SPRINGGREEN2: (u8, u8, u8) = (0, 238, 118);
pub const SPRINGGREEN3: (u8, u8, u8) = (0, 205, 102);
pub const SPRINGGREEN4: (u8, u8, u8) = (0, 139, 69);
pub const GREEN1: (u8, u8, u8) = (0, 255, 0);
pub const GREEN2: (u8, u8, u8) = (0, 238, 0);
pub const GREEN3: (u8, u8, u8) = (0, 205, 0);
pub const GREEN4: (u8, u8, u8) = (0, 139, 0);
pub const CHARTREUSE1: (u8, u8, u8) = (127, 255, 0);
pub const CHARTREUSE2: (u8, u8, u8) = (118, 238, 0);
pub const CHARTREUSE3: (u8, u8, u8) = (102, 205, 0);
pub const CHARTREUSE4: (u8, u8, u8) = (69, 139, 0);
pub const OLIVEDRAB1: (u8, u8, u8) = (192, 255, 62);
pub const OLIVEDRAB2: (u8, u8, u8) = (179, 238, 58);
pub const OLIVEDRAB3: (u8, u8, u8) = (154, 205, 50);
pub const OLIVEDRAB4: (u8, u8, u8) = (105, 139, 34);
pub const DARKOLIVEGREEN1: (u8, u8, u8) = (202, 255, 112);
pub const DARKOLIVEGREEN2: (u8, u8, u8) = (188, 238, 104);
pub const DARKOLIVEGREEN3: (u8, u8, u8) = (162, 205, 90);
pub const DARKOLIVEGREEN4: (u8, u8, u8) = (110, 139, 61);
pub const KHAKI1: (u8, u8, u8) = (255, 246, 143);
pub const KHAKI2: (u8, u8, u8) = (238, 230, 133);
pub const KHAKI3: (u8, u8, u8) = (205, 198, 115);
pub const KHAKI4: (u8, u8, u8) = (139, 134, 78);
pub const LIGHTGOLDENROD1: (u8, u8, u8) = (255, 236, 139);
pub const LIGHTGOLDENROD2: (u8, u8, u8) = (238, 220, 130);
pub const LIGHTGOLDENROD3: (u8, u8, u8) = (205, 190, 112);
pub const LIGHTGOLDENROD4: (u8, u8, u8) = (139, 129, 76);
pub const LIGHTYELLOW1: (u8, u8, u8) = (255, 255, 224);
pub const LIGHTYELLOW2: (u8, u8, u8) = (238, 238, 209);
pub const LIGHTYELLOW3: (u8, u8, u8) = (205, 205, 180);
pub const LIGHTYELLOW4: (u8, u8, u8) = (139, 139, 122);
pub const YELLOW1: (u8, u8, u8) = (255, 255, 0);
pub const YELLOW2: (u8, u8, u8) = (238, 238, 0);
pub const YELLOW3: (u8, u8, u8) = (205, 205, 0);
pub const YELLOW4: (u8, u8, u8) = (139, 139, 0);
pub const GOLD1: (u8, u8, u8) = (255, 215, 0);
pub const GOLD2: (u8, u8, u8) = (238, 201, 0);
pub const GOLD3: (u8, u8, u8) = (205, 173, 0);
pub const GOLD4: (u8, u8, u8) = (139, 117, 0);
pub const GOLDENROD1: (u8, u8, u8) = (255, 193, 37);
pub const GOLDENROD2: (u8, u8, u8) = (238, 180, 34);
pub const GOLDENROD3: (u8, u8, u8) = (205, 155, 29);
pub const GOLDENROD4: (u8, u8, u8) = (139, 105, 20);
pub const DARKGOLDENROD1: (u8, u8, u8) = (255, 185, 15);
pub const DARKGOLDENROD2: (u8, u8, u8) = (238, 173, 14);
pub const DARKGOLDENROD3: (u8, u8, u8) = (205, 149, 12);
pub const DARKGOLDENROD4: (u8, u8, u8) = (139, 101, 8);
pub const ROSYBROWN1: (u8, u8, u8) = (255, 193, 193);
pub const ROSYBROWN2: (u8, u8, u8) = (238, 180, 180);
pub const ROSYBROWN3: (u8, u8, u8) = (205, 155, 155);
pub const ROSYBROWN4: (u8, u8, u8) = (139, 105, 105);
pub const INDIANRED1: (u8, u8, u8) = (255, 106, 106);
pub const INDIANRED2: (u8, u8, u8) = (238, 99, 99);
pub const INDIANRED3: (u8, u8, u8) = (205, 85, 85);
pub const INDIANRED4: (u8, u8, u8) = (139, 58, 58);
pub const SIENNA1: (u8, u8, u8) = (255, 130, 71);
pub const SIENNA2: (u8, u8, u8) = (238, 121, 66);
pub const SIENNA3: (u8, u8, u8) = (205, 104, 57);
pub const SIENNA4: (u8, u8, u8) = (139, 71, 38);
pub const BURLYWOOD1: (u8, u8, u8) = (255, 211, 155);
pub const BURLYWOOD2: (u8, u8, u8) = (238, 197, 145);
pub const BURLYWOOD3: (u8, u8, u8) = (205, 170, 125);
pub const BURLYWOOD4: (u8, u8, u8) = (139, 115, 85);
pub const WHEAT1: (u8, u8, u8) = (255, 231, 186);
pub const WHEAT2: (u8, u8, u8) = (238, 216, 174);
pub const WHEAT3: (u8, u8, u8) = (205, 186, 150);
pub const WHEAT4: (u8, u8, u8) = (139, 126, 102);
pub const TAN1: (u8, u8, u8) = (255, 165, 79);
pub const TAN2: (u8, u8, u8) = (238, 154, 73);
pub const TAN3: (u8, u8, u8) = (205, 133, 63);
pub const TAN4: (u8, u8, u8) = (139, 90, 43);
pub const CHOCOLATE1: (u8, u8, u8) = (255, 127, 36);
pub const CHOCOLATE2: (u8, u8, u8) = (238, 118, 33);
pub const CHOCOLATE3: (u8, u8, u8) = (205, 102, 29);
pub const CHOCOLATE4: (u8, u8, u8) = (139, 69, 19);
pub const FIREBRICK1: (u8, u8, u8) = (255, 48, 48);
pub const FIREBRICK2: (u8, u8, u8) = (238, 44, 44);
pub const FIREBRICK3: (u8, u8, u8) = (205, 38, 38);
pub const FIREBRICK4: (u8, u8, u8) = (139, 26, 26);
pub const BROWN1: (u8, u8, u8) = (255, 64, 64);
pub const BROWN2: (u8, u8, u8) = (238, 59, 59);
pub const BROWN3: (u8, u8, u8) = (205, 51, 51);
pub const BROWN4: (u8, u8, u8) = (139, 35, 35);
pub const SALMON1: (u8, u8, u8) = (255, 140, 105);
pub const SALMON2: (u8, u8, u8) = (238, 130, 98);
pub const SALMON3: (u8, u8, u8) = (205, 112, 84);
pub const SALMON4: (u8, u8, u8) = (139, 76, 57);
pub const LIGHTSALMON1: (u8, u8, u8) = (255, 160, 122);
pub const LIGHTSALMON2: (u8, u8, u8) = (238, 149, 114);
pub const LIGHTSALMON3: (u8, u8, u8) = (205, 129, 98);
pub const LIGHTSALMON4: (u8, u8, u8) = (139, 87, 66);
pub const ORANGE1: (u8, u8, u8) = (255, 165, 0);
pub const ORANGE2: (u8, u8, u8) = (238, 154, 0);
pub const ORANGE3: (u8, u8, u8) = (205, 133, 0);
pub const ORANGE4: (u8, u8, u8) = (139, 90, 0);
pub const DARKORANGE1: (u8, u8, u8) = (255, 127, 0);
pub const DARKORANGE2: (u8, u8, u8) = (238, 118, 0);
pub const DARKORANGE3: (u8, u8, u8) = (205, 102, 0);
pub const DARKORANGE4: (u8, u8, u8) = (139, 69, 0);
pub const CORAL1: (u8, u8, u8) = (255, 114, 86);
pub const CORAL2: (u8, u8, u8) = (238, 106, 80);
pub const CORAL3: (u8, u8, u8) = (205, 91, 69);
pub const CORAL4: (u8, u8, u8) = (139, 62, 47);
pub const TOMATO1: (u8, u8, u8) = (255, 99, 71);
pub const TOMATO2: (u8, u8, u8) = (238, 92, 66);
pub const TOMATO3: (u8, u8, u8) = (205, 79, 57);
pub const TOMATO4: (u8, u8, u8) = (139, 54, 38);
pub const ORANGERED1: (u8, u8, u8) = (255, 69, 0);
pub const ORANGERED2: (u8, u8, u8) = (238, 64, 0);
pub const ORANGERED3: (u8, u8, u8) = (205, 55, 0);
pub const ORANGERED4: (u8, u8, u8) = (139, 37, 0);
pub const RED1: (u8, u8, u8) = (255, 0, 0);
pub const RED2: (u8, u8, u8) = (238, 0, 0);
pub const RED3: (u8, u8, u8) = (205, 0, 0);
pub const RED4: (u8, u8, u8) = (139, 0, 0);
pub const DEEPPINK1: (u8, u8, u8) = (255, 20, 147);
pub const DEEPPINK2: (u8, u8, u8) = (238, 18, 137);
pub const DEEPPINK3: (u8, u8, u8) = (205, 16, 118);
pub const DEEPPINK4: (u8, u8, u8) = (139, 10, 80);
pub const HOTPINK1: (u8, u8, u8) = (255, 110, 180);
pub const HOTPINK2: (u8, u8, u8) = (238, 106, 167);
pub const HOTPINK3: (u8, u8, u8) = (205, 96, 144);
pub const HOTPINK4: (u8, u8, u8) = (139, 58, 98);
pub const PINK1: (u8, u8, u8) = (255, 181, 197);
pub const PINK2: (u8, u8, u8) = (238, 169, 184);
pub const PINK3: (u8, u8, u8) = (205, 145, 158);
pub const PINK4: (u8, u8, u8) = (139, 99, 108);
pub const LIGHTPINK1: (u8, u8, u8) = (255, 174, 185);
pub const LIGHTPINK2: (u8, u8, u8) = (238, 162, 173);
pub const LIGHTPINK3: (u8, u8, u8) = (205, 140, 149);
pub const LIGHTPINK4: (u8, u8, u8) = (139, 95, 101);
pub const PALEVIOLETRED1: (u8, u8, u8) = (255, 130, 171);
pub const PALEVIOLETRED2: (u8, u8, u8) = (238, 121, 159);
pub const PALEVIOLETRED3: (u8, u8, u8) = (205, 104, 137);
pub const PALEVIOLETRED4: (u8, u8, u8) = (139, 71, 93);
pub const MAROON1: (u8, u8, u8) = (255, 52, 179);
pub const MAROON2: (u8, u8, u8) = (238, 48, 167);
pub const MAROON3: (u8, u8, u8) = (205, 41, 144);
pub const MAROON4: (u8, u8, u8) = (139, 28, 98);
pub const VIOLETRED1: (u8, u8, u8) = (255, 62, 150);
pub const VIOLETRED2: (u8, u8, u8) = (238, 58, 140);
pub const VIOLETRED3: (u8, u8, u8) = (205, 50, 120);
pub const VIOLETRED4: (u8, u8, u8) = (139, 34, 82);
pub const MAGENTA1: (u8, u8, u8) = (255, 0, 255);
pub const MAGENTA2: (u8, u8, u8) = (238, 0, 238);
pub const MAGENTA3: (u8, u8, u8) = (205, 0, 205);
pub const MAGENTA4: (u8, u8, u8) = (139, 0, 139);
pub const ORCHID1: (u8, u8, u8) = (255, 131, 250);
pub const ORCHID2: (u8, u8, u8) = (238, 122, 233);
pub const ORCHID3: (u8, u8, u8) = (205, 105, 201);
pub const ORCHID4: (u8, u8, u8) = (139, 71, 137);
pub const PLUM1: (u8, u8, u8) = (255, 187, 255);
pub const PLUM2: (u8, u8, u8) = (238, 174, 238);
pub const PLUM3: (u8, u8, u8) = (205, 150, 205);
pub const PLUM4: (u8, u8, u8) = (139, 102, 139);
pub const MEDIUMORCHID1: (u8, u8, u8) = (224, 102, 255);
pub const MEDIUMORCHID2: (u8, u8, u8) = (209, 95, 238);
pub const MEDIUMORCHID3: (u8, u8, u8) = (180, 82, 205);
pub const MEDIUMORCHID4: (u8, u8, u8) = (122, 55, 139);
pub const DARKORCHID1: (u8, u8, u8) = (191, 62, 255);
pub const DARKORCHID2: (u8, u8, u8) = (178, 58, 238);
pub const DARKORCHID3: (u8, u8, u8) = (154, 50, 205);
pub const DARKORCHID4: (u8, u8, u8) = (104, 34, 139);
pub const PURPLE1: (u8, u8, u8) = (155, 48, 255);
pub const PURPLE2: (u8, u8, u8) = (145, 44, 238);
pub const PURPLE3: (u8, u8, u8) = (125, 38, 205);
pub const PURPLE4: (u8, u8, u8) = (85, 26, 139);
pub const MEDIUMPURPLE1: (u8, u8, u8) = (171, 130, 255);
pub const MEDIUMPURPLE2: (u8, u8, u8) = (159, 121, 238);
pub const MEDIUMPURPLE3: (u8, u8, u8) = (137, 104, 205);
pub const MEDIUMPURPLE4: (u8, u8, u8) = (93, 71, 139);
pub const THISTLE1: (u8, u8, u8) = (255, 225, 255);
pub const THISTLE2: (u8, u8, u8) = (238, 210, 238);
pub const THISTLE3: (u8, u8, u8) = (205, 181, 205);
pub const THISTLE4: (u8, u8, u8) = (139, 123, 139);
pub const GRAY0: (u8, u8, u8) = (0, 0, 0);
pub const GREY0: (u8, u8, u8) = (0, 0, 0);
pub const GRAY1: (u8, u8, u8) = (3, 3, 3);
pub const GREY1: (u8, u8, u8) = (3, 3, 3);
pub const GRAY2: (u8, u8, u8) = (5, 5, 5);
pub const GREY2: (u8, u8, u8) = (5, 5, 5);
pub const GRAY3: (u8, u8, u8) = (8, 8, 8);
pub const GREY3: (u8, u8, u8) = (8, 8, 8);
pub const GRAY4: (u8, u8, u8) = (10, 10, 10);
pub const GREY4: (u8, u8, u8) = (10, 10, 10);
pub const GRAY5: (u8, u8, u8) = (13, 13, 13);
pub const GREY5: (u8, u8, u8) = (13, 13, 13);
pub const GRAY6: (u8, u8, u8) = (15, 15, 15);
pub const GREY6: (u8, u8, u8) = (15, 15, 15);
pub const GRAY7: (u8, u8, u8) = (18, 18, 18);
pub const GREY7: (u8, u8, u8) = (18, 18, 18);
pub const GRAY8: (u8, u8, u8) = (20, 20, 20);
pub const GREY8: (u8, u8, u8) = (20, 20, 20);
pub const GRAY9: (u8, u8, u8) = (23, 23, 23);
pub const GREY9: (u8, u8, u8) = (23, 23, 23);
pub const GRAY10: (u8, u8, u8) = (26, 26, 26);
pub const GREY10: (u8, u8, u8) = (26, 26, 26);
pub const GRAY11: (u8, u8, u8) = (28, 28, 28);
pub const GREY11: (u8, u8, u8) = (28, 28, 28);
pub const GRAY12: (u8, u8, u8) = (31, 31, 31);
pub const GREY12: (u8, u8, u8) = (31, 31, 31);
pub const GRAY13: (u8, u8, u8) = (33, 33, 33);
pub const GREY13: (u8, u8, u8) = (33, 33, 33);
pub const GRAY14: (u8, u8, u8) = (36, 36, 36);
pub const GREY14: (u8, u8, u8) = (36, 36, 36);
pub const GRAY15: (u8, u8, u8) = (38, 38, 38);
pub const GREY15: (u8, u8, u8) = (38, 38, 38);
pub const GRAY16: (u8, u8, u8) = (41, 41, 41);
pub const GREY16: (u8, u8, u8) = (41, 41, 41);
pub const GRAY17: (u8, u8, u8) = (43, 43, 43);
pub const GREY17: (u8, u8, u8) = (43, 43, 43);
pub const GRAY18: (u8, u8, u8) = (46, 46, 46);
pub const GREY18: (u8, u8, u8) = (46, 46, 46);
pub const GRAY19: (u8, u8, u8) = (48, 48, 48);
pub const GREY19: (u8, u8, u8) = (48, 48, 48);
pub const GRAY20: (u8, u8, u8) = (51, 51, 51);
pub const GREY20: (u8, u8, u8) = (51, 51, 51);
pub const GRAY21: (u8, u8, u8) = (54, 54, 54);
pub const GREY21: (u8, u8, u8) = (54, 54, 54);
pub const GRAY22: (u8, u8, u8) = (56, 56, 56);
pub const GREY22: (u8, u8, u8) = (56, 56, 56);
pub const GRAY23: (u8, u8, u8) = (59, 59, 59);
pub const GREY23: (u8, u8, u8) = (59, 59, 59);
pub const GRAY24: (u8, u8, u8) = (61, 61, 61);
pub const GREY24: (u8, u8, u8) = (61, 61, 61);
pub const GRAY25: (u8, u8, u8) = (64, 64, 64);
pub const GREY25: (u8, u8, u8) = (64, 64, 64);
pub const GRAY26: (u8, u8, u8) = (66, 66, 66);
pub const GREY26: (u8, u8, u8) = (66, 66, 66);
pub const GRAY27: (u8, u8, u8) = (69, 69, 69);
pub const GREY27: (u8, u8, u8) = (69, 69, 69);
pub const GRAY28: (u8, u8, u8) = (71, 71, 71);
pub const GREY28: (u8, u8, u8) = (71, 71, 71);
pub const GRAY29: (u8, u8, u8) = (74, 74, 74);
pub const GREY29: (u8, u8, u8) = (74, 74, 74);
pub const GRAY30: (u8, u8, u8) = (77, 77, 77);
pub const GREY30: (u8, u8, u8) = (77, 77, 77);
pub const GRAY31: (u8, u8, u8) = (79, 79, 79);
pub const GREY31: (u8, u8, u8) = (79, 79, 79);
pub const GRAY32: (u8, u8, u8) = (82, 82, 82);
pub const GREY32: (u8, u8, u8) = (82, 82, 82);
pub const GRAY33: (u8, u8, u8) = (84, 84, 84);
pub const GREY33: (u8, u8, u8) = (84, 84, 84);
pub const GRAY34: (u8, u8, u8) = (87, 87, 87);
pub const GREY34: (u8, u8, u8) = (87, 87, 87);
pub const GRAY35: (u8, u8, u8) = (89, 89, 89);
pub const GREY35: (u8, u8, u8) = (89, 89, 89);
pub const GRAY36: (u8, u8, u8) = (92, 92, 92);
pub const GREY36: (u8, u8, u8) = (92, 92, 92);
pub const GRAY37: (u8, u8, u8) = (94, 94, 94);
pub const GREY37: (u8, u8, u8) = (94, 94, 94);
pub const GRAY38: (u8, u8, u8) = (97, 97, 97);
pub const GREY38: (u8, u8, u8) = (97, 97, 97);
pub const GRAY39: (u8, u8, u8) = (99, 99, 99);
pub const GREY39: (u8, u8, u8) = (99, 99, 99);
pub const GRAY40: (u8, u8, u8) = (102, 102, 102);
pub const GREY40: (u8, u8, u8) = (102, 102, 102);
pub const GRAY41: (u8, u8, u8) = (105, 105, 105);
pub const GREY41: (u8, u8, u8) = (105, 105, 105);
pub const GRAY42: (u8, u8, u8) = (107, 107, 107);
pub const GREY42: (u8, u8, u8) = (107, 107, 107);
pub const GRAY43: (u8, u8, u8) = (110, 110, 110);
pub const GREY43: (u8, u8, u8) = (110, 110, 110);
pub const GRAY44: (u8, u8, u8) = (112, 112, 112);
pub const GREY44: (u8, u8, u8) = (112, 112, 112);
pub const GRAY45: (u8, u8, u8) = (115, 115, 115);
pub const GREY45: (u8, u8, u8) = (115, 115, 115);
pub const GRAY46: (u8, u8, u8) = (117, 117, 117);
pub const GREY46: (u8, u8, u8) = (117, 117, 117);
pub const GRAY47: (u8, u8, u8) = (120, 120, 120);
pub const GREY47: (u8, u8, u8) = (120, 120, 120);
pub const GRAY48: (u8, u8, u8) = (122, 122, 122);
pub const GREY48: (u8, u8, u8) = (122, 122, 122);
pub const GRAY49: (u8, u8, u8) = (125, 125, 125);
pub const GREY49: (u8, u8, u8) = (125, 125, 125);
pub const GRAY50: (u8, u8, u8) = (127, 127, 127);
pub const GREY50: (u8, u8, u8) = (127, 127, 127);
pub const GRAY51: (u8, u8, u8) = (130, 130, 130);
pub const GREY51: (u8, u8, u8) = (130, 130, 130);
pub const GRAY52: (u8, u8, u8) = (133, 133, 133);
pub const GREY52: (u8, u8, u8) = (133, 133, 133);
pub const GRAY53: (u8, u8, u8) = (135, 135, 135);
pub const GREY53: (u8, u8, u8) = (135, 135, 135);
pub const GRAY54: (u8, u8, u8) = (138, 138, 138);
pub const GREY54: (u8, u8, u8) = (138, 138, 138);
pub const GRAY55: (u8, u8, u8) = (140, 140, 140);
pub const GREY55: (u8, u8, u8) = (140, 140, 140);
pub const GRAY56: (u8, u8, u8) = (143, 143, 143);
pub const GREY56: (u8, u8, u8) = (143, 143, 143);
pub const GRAY57: (u8, u8, u8) = (145, 145, 145);
pub const GREY57: (u8, u8, u8) = (145, 145, 145);
pub const GRAY58: (u8, u8, u8) = (148, 148, 148);
pub const GREY58: (u8, u8, u8) = (148, 148, 148);
pub const GRAY59: (u8, u8, u8) = (150, 150, 150);
pub const GREY59: (u8, u8, u8) = (150, 150, 150);
pub const GRAY60: (u8, u8, u8) = (153, 153, 153);
pub const GREY60: (u8, u8, u8) = (153, 153, 153);
pub const GRAY61: (u8, u8, u8) = (156, 156, 156);
pub const GREY61: (u8, u8, u8) = (156, 156, 156);
pub const GRAY62: (u8, u8, u8) = (158, 158, 158);
pub const GREY62: (u8, u8, u8) = (158, 158, 158);
pub const GRAY63: (u8, u8, u8) = (161, 161, 161);
pub const GREY63: (u8, u8, u8) = (161, 161, 161);
pub const GRAY64: (u8, u8, u8) = (163, 163, 163);
pub const GREY64: (u8, u8, u8) = (163, 163, 163);
pub const GRAY65: (u8, u8, u8) = (166, 166, 166);
pub const GREY65: (u8, u8, u8) = (166, 166, 166);
pub const GRAY66: (u8, u8, u8) = (168, 168, 168);
pub const GREY66: (u8, u8, u8) = (168, 168, 168);
pub const GRAY67: (u8, u8, u8) = (171, 171, 171);
pub const GREY67: (u8, u8, u8) = (171, 171, 171);
pub const GRAY68: (u8, u8, u8) = (173, 173, 173);
pub const GREY68: (u8, u8, u8) = (173, 173, 173);
pub const GRAY69: (u8, u8, u8) = (176, 176, 176);
pub const GREY69: (u8, u8, u8) = (176, 176, 176);
pub const GRAY70: (u8, u8, u8) = (179, 179, 179);
pub const GREY70: (u8, u8, u8) = (179, 179, 179);
pub const GRAY71: (u8, u8, u8) = (181, 181, 181);
pub const GREY71: (u8, u8, u8) = (181, 181, 181);
pub const GRAY72: (u8, u8, u8) = (184, 184, 184);
pub const GREY72: (u8, u8, u8) = (184, 184, 184);
pub const GRAY73: (u8, u8, u8) = (186, 186, 186);
pub const GREY73: (u8, u8, u8) = (186, 186, 186);
pub const GRAY74: (u8, u8, u8) = (189, 189, 189);
pub const GREY74: (u8, u8, u8) = (189, 189, 189);
pub const GRAY75: (u8, u8, u8) = (191, 191, 191);
pub const GREY75: (u8, u8, u8) = (191, 191, 191);
pub const GRAY76: (u8, u8, u8) = (194, 194, 194);
pub const GREY76: (u8, u8, u8) = (194, 194, 194);
pub const GRAY77: (u8, u8, u8) = (196, 196, 196);
pub const GREY77: (u8, u8, u8) = (196, 196, 196);
pub const GRAY78: (u8, u8, u8) = (199, 199, 199);
pub const GREY78: (u8, u8, u8) = (199, 199, 199);
pub const GRAY79: (u8, u8, u8) = (201, 201, 201);
pub const GREY79: (u8, u8, u8) = (201, 201, 201);
pub const GRAY80: (u8, u8, u8) = (204, 204, 204);
pub const GREY80: (u8, u8, u8) = (204, 204, 204);
pub const GRAY81: (u8, u8, u8) = (207, 207, 207);
pub const GREY81: (u8, u8, u8) = (207, 207, 207);
pub const GRAY82: (u8, u8, u8) = (209, 209, 209);
pub const GREY82: (u8, u8, u8) = (209, 209, 209);
pub const GRAY83: (u8, u8, u8) = (212, 212, 212);
pub const GREY83: (u8, u8, u8) = (212, 212, 212);
pub const GRAY84: (u8, u8, u8) = (214, 214, 214);
pub const GREY84: (u8, u8, u8) = (214, 214, 214);
pub const GRAY85: (u8, u8, u8) = (217, 217, 217);
pub const GREY85: (u8, u8, u8) = (217, 217, 217);
pub const GRAY86: (u8, u8, u8) = (219, 219, 219);
pub const GREY86: (u8, u8, u8) = (219, 219, 219);
pub const GRAY87: (u8, u8, u8) = (222, 222, 222);
pub const GREY87: (u8, u8, u8) = (222, 222, 222);
pub const GRAY88: (u8, u8, u8) = (224, 224, 224);
pub const GREY88: (u8, u8, u8) = (224, 224, 224);
pub const GRAY89: (u8, u8, u8) = (227, 227, 227);
pub const GREY89: (u8, u8, u8) = (227, 227, 227);
pub const GRAY90: (u8, u8, u8) = (229, 229, 229);
pub const GREY90: (u8, u8, u8) = (229, 229, 229);
pub const GRAY91: (u8, u8, u8) = (232, 232, 232);
pub const GREY91: (u8, u8, u8) = (232, 232, 232);
pub const GRAY92: (u8, u8, u8) = (235, 235, 235);
pub const GREY92: (u8, u8, u8) = (235, 235, 235);
pub const GRAY93: (u8, u8, u8) = (237, 237, 237);
pub const GREY93: (u8, u8, u8) = (237, 237, 237);
pub const GRAY94: (u8, u8, u8) = (240, 240, 240);
pub const GREY94: (u8, u8, u8) = (240, 240, 240);
pub const GRAY95: (u8, u8, u8) = (242, 242, 242);
pub const GREY95: (u8, u8, u8) = (242, 242, 242);
pub const GRAY96: (u8, u8, u8) = (245, 245, 245);
pub const GREY96: (u8, u8, u8) = (245, 245, 245);
pub const GRAY97: (u8, u8, u8) = (247, 247, 247);
pub const GREY97: (u8, u8, u8) = (247, 247, 247);
pub const GRAY98: (u8, u8, u8) = (250, 250, 250);
pub const GREY98: (u8, u8, u8) = (250, 250, 250);
pub const GRAY99: (u8, u8, u8) = (252, 252, 252);
pub const GREY99: (u8, u8, u8) = (252, 252, 252);
pub const GRAY100: (u8, u8, u8) = (255, 255, 255);
pub const GREY100: (u8, u8, u8) = (255, 255, 255);
pub const DARK_GREY: (u8, u8, u8) = (169, 169, 169);
pub const DARKGREY: (u8, u8, u8) = (169, 169, 169);
pub const DARK_GRAY: (u8, u8, u8) = (169, 169, 169);
pub const DARKGRAY: (u8, u8, u8) = (169, 169, 169);
pub const DARK_BLUE: (u8, u8, u8) = (0, 0, 139);
pub const DARKBLUE: (u8, u8, u8) = (0, 0, 139);
pub const DARK_CYAN: (u8, u8, u8) = (0, 139, 139);
pub const DARKCYAN: (u8, u8, u8) = (0, 139, 139);
pub const DARK_MAGENTA: (u8, u8, u8) = (139, 0, 139);
pub const DARKMAGENTA: (u8, u8, u8) = (139, 0, 139);
pub const DARK_RED: (u8, u8, u8) = (139, 0, 0);
pub const DARKRED: (u8, u8, u8) = (139, 0, 0);
pub const LIGHT_GREEN: (u8, u8, u8) = (144, 238, 144);
pub const LIGHTGREEN: (u8, u8, u8) = (144, 238, 144);
pub const CRIMSON: (u8, u8, u8) = (220, 20, 60);
pub const INDIGO: (u8, u8, u8) = (75, 0, 130);
pub const OLIVE: (u8, u8, u8) = (128, 128, 0);
pub const REBECCA_PURPLE: (u8, u8, u8) = (102, 51, 153);
pub const REBECCAPURPLE: (u8, u8, u8) = (102, 51, 153);
pub const SILVER: (u8, u8, u8) = (192, 192, 192);
pub const TEAL: (u8, u8, u8) = (0, 128, 128);

// Unit tests for the color system

#[cfg(test)]
mod tests {
    use super::HSV;
    use super::RGB;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn make_rgb_minimal() {
        let black = RGB::new();
        assert!(black.r < std::f32::EPSILON);
        assert!(black.g < std::f32::EPSILON);
        assert!(black.b < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn make_hsv_minimal() {
        let black = HSV::new();
        assert!(black.h < std::f32::EPSILON);
        assert!(black.s < std::f32::EPSILON);
        assert!(black.v < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_red_to_hsv() {
        let red = RGB::from_f32(1.0, 0.0, 0.0);
        let hsv = red.to_hsv();
        assert!(hsv.h < std::f32::EPSILON);
        assert!(f32::abs(hsv.s - 1.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.v - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_green_to_hsv() {
        let green = RGB::from_f32(0.0, 1.0, 0.0);
        let hsv = green.to_hsv();
        assert!(f32::abs(hsv.h - 120.0 / 360.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.s - 1.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.v - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_blue_to_hsv() {
        let blue = RGB::from_f32(0.0, 0.0, 1.0);
        let hsv = blue.to_hsv();
        assert!(f32::abs(hsv.h - 240.0 / 360.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.s - 1.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.v - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_olive_to_hsv() {
        let grey = RGB::from_u8(128, 128, 0);
        let hsv = grey.to_hsv();
        assert!(f32::abs(hsv.h - 60.0 / 360.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.s - 1.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.v - 0.5019_608) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_olive_to_rgb() {
        let grey = HSV::from_f32(60.0 / 360.0, 1.0, 0.501_960_8);
        let rgb = grey.to_rgb();
        assert!(f32::abs(rgb.r - 128.0 / 255.0) < std::f32::EPSILON);
        assert!(f32::abs(rgb.g - 128.0 / 255.0) < std::f32::EPSILON);
        assert!(rgb.b < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_red_hex() {
        let rgb = RGB::from_hex("#FF0000").expect("Invalid hex string");
        assert!(f32::abs(rgb.r - 1.0) < std::f32::EPSILON);
        assert!(rgb.g < std::f32::EPSILON);
        assert!(rgb.b < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_green_hex() {
        let rgb = RGB::from_hex("#00FF00").expect("Invalid hex string");
        assert!(rgb.r < std::f32::EPSILON);
        assert!(f32::abs(rgb.g - 1.0) < std::f32::EPSILON);
        assert!(rgb.b < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_blue_hex() {
        let rgb = RGB::from_hex("#0000FF").expect("Invalid hex string");
        assert!(rgb.r < std::f32::EPSILON);
        assert!(rgb.g < std::f32::EPSILON);
        assert!(f32::abs(rgb.b - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_blue_named() {
        let rgb = RGB::named(super::BLUE);
        assert!(rgb.r < std::f32::EPSILON);
        assert!(rgb.g < std::f32::EPSILON);
        assert!(f32::abs(rgb.b - 1.0) < std::f32::EPSILON);
    }
}
