#![allow(clippy::many_single_char_names)]

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

    pub fn to_druid(&self) -> druid::Color {
        let [r, g, b, a] = self.pixel();
        druid::Color::rgba8(r, g, b, a)
    }

    pub fn pixel(&self) -> [u8; 4] {
        [u(self.rgb.0), u(self.rgb.1), u(self.rgb.2), u(self.a)]
    }

    pub fn to_hex_string(&self) -> String {
        let [r, g, b, a] = self.pixel();
        if a == 255 {
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
        }
    }
    pub fn to_rgb_string(&self) -> String {
        let [r, g, b, a] = self.pixel();
        if a == 255 {
            format!("rgb({}, {}, {})", r, g, b)
        } else {
            format!("rgba({}, {}, {}, {:.0}%)", r, g, b, a as f32 / 255.0 * 100.0)
        }
    }
    pub fn to_hsv_string(&self) -> String {
        let h = self.hsv.0 * 360.0;
        let s = self.hsv.1 * 100.0;
        let v = self.hsv.2 * 100.0;
        let a = self.a * 100.0;
        if feq(a, 100.0) {
            format!("hsv({:.0}deg, {:.0}%, {:.0}%)", h, s, v)
        } else {
            format!("hsva({:.0}deg, {:.0}%, {:.0}%, {:.0}%)", h, s, v, a)
        }
    }

    pub fn to_hsl_string(&self) -> String {
        let (h, s, l) = hsv_to_hsl(self.hsv.0, self.hsv.1, self.hsv.2);
        let h = h * 360.0;
        let s = s * 100.0;
        let l = l * 100.0;
        let a = self.a * 100.0;
        if feq(a, 100.0) {
            format!("hsl({:.0}deg, {:.0}%, {:.0}%)", h, s, l)
        } else {
            format!("hsla({:.0}deg, {:.0}%, {:.0}%, {:.0}%)", h, s, l, a)
        }
    }

    pub fn to_vec_string(&self) -> String {
        if feq(self.a, 1.0) {
            format!("vec3({:.2}, {:.2}, {:.2})", self.rgb.0, self.rgb.1, self.rgb.2)
        } else {
            format!("vec4({:.2}, {:.2}, {:.2}, {:.2})", self.rgb.0, self.rgb.1, self.rgb.2, self.a)
        }
    }
}

fn u(x: f32) -> u8 {
    (x * 255.).round() as u8
}

fn feq(x: f32, y: f32) -> bool {
    (x - y).abs() <= f32::EPSILON
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

    let h = if feq(c, 0.0) {
        0.0
    } else if feq(v, r) {
        (g - b) / c
    } else if feq(v, g) {
        (b - r) / c + 2.0
    } else {
        (r - g) / c + 4.0
    };

    let s = if feq(v, 0.0) {
        0.0
    } else {
        c / v
    };

    (h, s, v)
}

// https://en.wikipedia.org/wiki/HSL_and_HSV#Interconversion
fn hsv_to_hsl(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let l = v * (1.0 - s/2.0);
    let s = if feq(l, 0.0) || feq(l, 1.0) {
        0.0
    } else {
        (v - l) / l.min(1.0 - l)
    };
    (h, s, l)
}

#[cfg(test)]
mod tests {
    #[test]
    fn hsv_to_rgb() {
        assert_eq!(super::hsv_to_rgb(30.0/360.0, 1.0, 1.0), (1.0, 0.5, 0.0));
        assert_eq!(super::hsv_to_rgb(60.0/360.0, 0.5, 0.75), (0.75, 0.75, 0.375));
    }
}