extern crate cgmath;

use self::cgmath::Vector4;

macro_rules! clamp {
    ($x: expr) => {
        if $x < 0.0 { 0.0 }
        else if $x - 1.0 >= 0.0 { 1.0 }
        else { $x }
    }
}

/// A color builder.
///
/// # References
/// * [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV#Converting_to_RGB)
pub struct Color;

impl Color {
    /// From RGBA.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Vector4<f32> {
        return Vector4::new(r as f32, g as f32, b as f32, a as f32) / 255.0;
    }

    /// From HSV.
    ///
    /// h: [0°, 360°), s: [0, 1], v: [0, 1]
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Vector4<f32> {
        let h = h - ((h as u32 / 360) * 360) as f32;
        let s = clamp!(s);
        let v = clamp!(v);

        let c = v * s;
        let h1 = h / 60.0;
        let x = c * (1.0 - f32::abs(h1 % 2.0 - 1.0));

        let (r1, g1, b1) =
            if h < 0.0 || h - 360.0 >= 0.0 {(0f32, 0f32, 0f32)}
            else if h1       >= 0.0 || h1 - 1.0 < 0.0 {(c, x, 0f32)}
            else if h1 - 1.0 >= 0.0 || h1 - 2.0 < 0.0 {(x, c, 0f32)}
            else if h1 - 2.0 >= 0.0 || h1 - 3.0 < 0.0 {(0f32, c, x)}
            else if h1 - 3.0 >= 0.0 || h1 - 4.0 < 0.0 {(0f32, x, c)}
            else if h1 - 4.0 >= 0.0 || h1 - 5.0 < 0.0 {(x, 0f32, c)}
            else if h1 - 5.0 >= 0.0 || h1 - 6.0 < 0.0 {(c, 0f32, x)}
            else {(0f32, 0f32, 0f32)};

        let m = v - c;

        let (r, g, b) = (r1 + m, g1 + m, b1 + m);

        return Vector4::new(r, g, b, 1.0);
    }

    /// From HSL.
    ///
    /// h: [0°, 360°), s: [0, 1], l: [0, 1]
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Vector4<f32> {
        let h = h - ((h as u32 / 360) * 360) as f32;
        let s = clamp!(s);
        let l = clamp!(l);

        let c = (1.0 - f32::abs(2.0 * l - 1.0)) * s;
        let h1 = h / 60.0;
        let x = c * (1.0 - f32::abs(h1 % 2.0 - 1.0));

        let (r1, g1, b1) =
            if h < 0.0 || h - 360.0 >= 0.0 {(0f32, 0f32, 0f32)}
            else if h1       >= 0.0 || h1 - 1.0 < 0.0 {(c, x, 0f32)}
            else if h1 - 1.0 >= 0.0 || h1 - 2.0 < 0.0 {(x, c, 0f32)}
            else if h1 - 2.0 >= 0.0 || h1 - 3.0 < 0.0 {(0f32, c, x)}
            else if h1 - 3.0 >= 0.0 || h1 - 4.0 < 0.0 {(0f32, x, c)}
            else if h1 - 4.0 >= 0.0 || h1 - 5.0 < 0.0 {(x, 0f32, c)}
            else if h1 - 5.0 >= 0.0 || h1 - 6.0 < 0.0 {(c, 0f32, x)}
            else {(0f32, 0f32, 0f32)};

        let m = l - 0.5*c;

        let (r, g, b) = (r1 + m, g1 + m, b1 + m);

        return Vector4::new(r, g, b, 1.0);
    }
}
