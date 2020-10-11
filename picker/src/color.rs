#![allow(many_single_char_names)]

use std::str::FromStr;
use druid::Data;

#[derive(Debug, Data, Clone)]
struct Rgb(f32, f32, f32);

impl Rgb {
    fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let (r, g, b) = hsv_to_rgb(h, s, v);
        Self(r, g, b)
    }
}

#[derive(Debug, Data, Clone)]
struct Hsv(f32, f32, f32);

impl Hsv {
    fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        let (h, s, l) = rgb_to_hsv(r, g, b);
        Self(h, s, l)
    }
}

#[derive(Debug, Clone, Data)]
pub struct Color {
    rgb: Rgb,
    hsv: Hsv,
    a: f32,
}

impl Color {
    pub fn from_hsva_f32(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self{
            rgb: Rgb::from_hsv(h, s, v),
            hsv: Hsv(h, s, v),
            a
        }
    }
    pub fn from_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self{
            rgb: Rgb(r, g, b),
            hsv: Hsv::from_rgb(r, g, b),
            a
        }
    }

    pub fn hue(&self) -> f32 {
        self.hsv.0
    }
    pub fn saturation(&self) -> f32 {
        self.hsv.1
    }
    pub fn value(&self) -> f32 {
        self.hsv.2
    }
    pub fn alpha(&self) -> f32 {
        self.a
    }

    pub fn set_hue(&mut self, h: f32) {
        self.hsv.0 = h;
        self.rgb = Rgb::from_hsv(self.hsv.0, self.hsv.1, self.hsv.2);
    }
    pub fn set_saturation(&mut self, s: f32) {
        self.hsv.1 = s;
        self.rgb = Rgb::from_hsv(self.hsv.0, self.hsv.1, self.hsv.2);
    }
    pub fn set_value(&mut self, v: f32) {
        self.hsv.2 = v;
        self.rgb = Rgb::from_hsv(self.hsv.0, self.hsv.1, self.hsv.2);
    }
    pub fn set_alpha(&mut self, a: f32) {
        self.a = a;
    }

    pub fn hex(&self) -> String {
        let [r, g, b, a] = self.pixel();
        if a == 255 {
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
        }
    }

    pub fn to_druid(&self) -> druid::Color {
        let [r, g, b, a] = self.pixel();
        druid::Color::rgba8(r, g, b, a)
    }

    pub fn pixel(&self) -> [u8; 4] {
        [u(self.rgb.0), u(self.rgb.1), u(self.rgb.2), u(self.a)]
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(|c: css_color::Rgba| Color::from_rgba_f32(c.red, c.green, c.blue, c.alpha))
            .map_err(|e| format!("{:?}", e))
    }
}

fn u(x: f32) -> u8 {
    (x * 255.).round() as u8
}

// https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let h = h * 6.0;
    let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());
    
    let (r, g, b) = if h <= 1.0 {
        (c, x, 0.0)
    } else if h <= 2.0 {
        (x, c, 0.0)
    } else if h <= 3.0 {
        (0.0, c, x)
    } else if h <= 4.0 {
        (0.0, x, c)
    } else if h <= 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    let m = v - c;
    (r+m, g+m, b+m)
}

// https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let v = r.max(g).max(b);
    let min = r.min(g).min(b);
    let c = v - min;

    let h = if c <= f32::EPSILON {
        0.0
    } else if (v - r).abs() <= f32::EPSILON {
        (g - b) / c
    } else if (v - g).abs() <= f32::EPSILON {
        (b - r) / c + 2.0
    } else {
        (r - g) / c + 4.0
    };

    let s = if v <= f32::EPSILON {
        0.0
    } else {
        c / v
    };

    (h, s, v)
}

#[cfg(test)]
mod tests {

    #[test]
    fn hsv_to_rgb() {
        assert_eq!(super::hsv_to_rgb(30.0/360.0, 1.0, 1.0), (1.0, 0.5, 0.0));
        assert_eq!(super::hsv_to_rgb(60.0/360.0, 0.5, 0.75), (0.75, 0.75, 0.375));
    }
}