use std::str::FromStr;
use std::fmt::LowerHex;
use druid::Data;

#[derive(Debug, Data, Clone)]
struct Rgb(f32, f32, f32);

impl Rgb {
    fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        Self(r, g, b)
    }
}

#[derive(Debug, Data, Clone)]
struct Hsl(f32, f32, f32);

impl Hsl {
    fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        let (h, s, l) = rgb_to_hsl(r, g, b);
        Self(h, s, l)
    }
}

#[derive(Debug, Clone, Data)]
pub struct Color {
    rgb: Rgb,
    hsl: Hsl,
    a: f32,
}

impl Color {
    pub fn from_hsla_f32(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self{
            rgb: Rgb::from_hsl(h, s, l),
            hsl: Hsl(h, s, l),
            a
        }
    }
    pub fn from_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self{
            rgb: Rgb(r, g, b),
            hsl: Hsl::from_rgb(r, g, b),
            a
        }
    }

    pub fn hue(&self) -> f32 {
        self.hsl.0
    }
    pub fn saturation(&self) -> f32 {
        self.hsl.1
    }
    pub fn lightness(&self) -> f32 {
        self.hsl.2
    }
    pub fn alpha(&self) -> f32 {
        self.a
    }

    pub fn set_hue(&mut self, h: f32) {
        self.hsl.0 = h;
        self.rgb = Rgb::from_hsl(self.hsl.0, self.hsl.1, self.hsl.2);
    }
    pub fn set_saturation(&mut self, s: f32) {
        self.hsl.1 = s;
        self.rgb = Rgb::from_hsl(self.hsl.0, self.hsl.1, self.hsl.2);
    }
    pub fn set_lightness(&mut self, l: f32) {
        self.hsl.2 = l;
        self.rgb = Rgb::from_hsl(self.hsl.0, self.hsl.1, self.hsl.2);
    }
    pub fn set_alpha(&mut self, a: f32) {
        self.a = a;
    }

    pub fn to_rgba_f32_tuple(&self) -> (f32, f32, f32, f32) {
        (self.rgb.0, self.rgb.1, self.rgb.2, self.a)
    }

    pub fn to_rgba_u32(&self) -> u32 {
        let [r, g, b, a] = self.pixel();

        // 0xRRGGBBAA
        ((r as u32) << 24) + ((g as u32) << 16) + ((b as u32) << 8) + (a as u32)
    }

    pub fn hex(&self) -> String {
        format!("#{:x}", self)
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

impl LowerHex for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.to_rgba_u32())
    }
}

fn u(x: f32) -> u8 {
    (x * 255.).round() as u8
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let mut t = t;
    if t < 0. {
        t += 1.
    }
    if t > 1. {
        t -= 1.
    }
    if t < 1. / 6. {
        return p + (q - p) * 6. * t;
    }
    if t < 1. / 2. {
        return q;
    }
    if t < 2. / 3. {
        return p + (q - p) * (2. / 3. - t) * 6.;
    }
    p
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }

    let q = if l < 0.5 {
        l * (1. + s)
    } else {
        l + s - l * s
    };
    let p = 2. * l - q;
    (
        hue_to_rgb(p, q, h + 1. / 3.),
        hue_to_rgb(p, q, h),
        hue_to_rgb(p, q, h - 1. / 3.),
    )
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < f32::EPSILON {
        return (0.0, 0.0, l)
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = if (max - r).abs() < f32::EPSILON {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    (h / 6.0, s, l)
}