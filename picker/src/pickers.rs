use crate::color::Color;
use druid::{BoxConstraints, Cursor, Env, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget};
use druid::kurbo::{self, Circle};
use druid::piet::{ImageFormat, InterpolationMode};

pub struct SatLightPicker {
    size: Size,
}

impl SatLightPicker {
    pub fn new() -> Self { Self { size: Size::new(0.0, 0.0) } }

    fn set(&self, p: Point, c: &mut Color) {
        c.set_saturation((p.x.max(0.0).min(self.size.width) / self.size.width) as f32);
        c.set_lightness((p.y.max(0.0).min(self.size.height) / self.size.height) as f32);
    }
}

impl Widget<Color> for SatLightPicker {
    fn paint(&mut self, ctx: &mut PaintCtx, data: &Color, _env: &Env) {
        self.size = ctx.size();
        let width = self.size.width.floor() as usize;
        let height = self.size.height.floor() as usize;

        let buf = draw(width, height, |x, y| {
            let sat = x as f32 / width as f32;
            let light = y as f32 / width as f32;
            Color::from_hsla_f32(data.hue(), sat, light, 1.0).pixel()
        });

        let image = ctx
            .make_image(width, height, &buf, ImageFormat::RgbaSeparate)
            .unwrap();

        ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, self.size),
            InterpolationMode::Bilinear,
        );

        let x = data.saturation() as f64 * width as f64;
        let y = data.lightness() as f64 * height as f64;
        let size = 4.0;
        let stroke = 2.0;
        let circle = Circle::new(Point::new(x, y), size);
        ctx.stroke(circle, &druid::Color::WHITE, stroke);
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &Color, _env: &Env) -> druid::Size {
        bc.max()
    }
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut Color, _env: &Env) {
        match event {
            druid::Event::MouseDown(e) => {
                ctx.set_active(true);
                self.set(e.pos, data);
            }
            druid::Event::MouseUp(_) => {
                ctx.set_active(false);
            }
            druid::Event::MouseMove(e) => {
                ctx.set_cursor(&Cursor::Crosshair);
                if ctx.is_active() {
                    self.set(e.pos, data);
                }
            }
            _ => ()
        }
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &druid::LifeCycle, _data: &Color, _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Color, _data: &Color, _env: &Env) {}
}

pub struct HuePicker {
    size: Size,
}

impl HuePicker {
    pub fn new() -> Self { Self { size: Size::new(0.0, 0.0) } }

    fn set(&self, p: Point, c: &mut Color) {
        c.set_hue((p.y.max(0.0).min(self.size.height) / self.size.height) as f32);
    }
}

impl Widget<Color> for HuePicker {
    fn paint(&mut self, ctx: &mut PaintCtx, data: &Color, _env: &Env) {
        self.size = ctx.size();
        let width = self.size.width.floor() as usize;
        let height = self.size.height.floor() as usize;

        let buf = draw(width, height, |_x, y| {
            let hue = y as f32 / height as f32;
            Color::from_hsla_f32(hue, data.saturation(), data.lightness(), 1.0).pixel()
        });

        let image = ctx
            .make_image(width, height, &buf, ImageFormat::RgbaSeparate)
            .unwrap();

        ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, self.size),
            InterpolationMode::Bilinear,
        );

        let y = data.hue() as f64 * height as f64;
        let size = 4.0;
        let inset = 1.0;
        let stroke = 2.0;
        let rect = kurbo::Rect::new(inset, y, width as f64 - inset, y+size);
        ctx.stroke(rect, &druid::Color::WHITE, stroke);
    }

    fn layout( &mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &Color, _env: &Env ) -> druid::Size {
        bc.max()
    }
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut Color, _env: &Env) {
        match event {
            druid::Event::MouseDown(e) => {
                ctx.set_active(true);
                self.set(e.pos, data);
            }
            druid::Event::MouseUp(_) => {
                ctx.set_active(false);
            }
            druid::Event::MouseMove(e) => {
                ctx.set_cursor(&Cursor::OpenHand);
                if ctx.is_active() {
                    self.set(e.pos, data);
                }
            }
            _ => ()
        }
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &druid::LifeCycle, _data: &Color, _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Color, _data: &Color, _env: &Env) {}
}

pub struct AlphaPicker {
    size: Size,
}

impl AlphaPicker {
    pub fn new() -> Self { Self { size: Size::new(0.0, 0.0) } }

    fn set(&self, p: Point, c: &mut Color) {
        c.set_alpha(1.0 - (p.y.max(0.0).min(self.size.height) / self.size.height) as f32);
    }
}

impl Widget<Color> for AlphaPicker {
    fn paint(&mut self, ctx: &mut PaintCtx, data: &Color, _env: &Env) {
        self.size = ctx.size();
        let width = self.size.width.floor() as usize;
        let height = self.size.height.floor() as usize;

        let buf = draw(width, height, |_x, y| {
            let alpha = 1.0 - y as f32 / height as f32;
            Color::from_hsla_f32(data.hue(), data.saturation(), data.lightness(), alpha).pixel()
        });

        let image = ctx
            .make_image(width, height, &buf, ImageFormat::RgbaSeparate)
            .unwrap();

        ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, self.size),
            InterpolationMode::Bilinear,
        );

        let y = (1.0 - data.alpha()) as f64 * height as f64;
        let size = 4.0;
        let inset = 1.0;
        let stroke = 2.0;
        let rect = kurbo::Rect::new(inset, y, width as f64 - inset, y+size);
        ctx.stroke(rect, &druid::Color::WHITE, stroke);
    }

    fn layout( &mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &Color, _env: &Env ) -> druid::Size {
        bc.max()
    }
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut Color, _env: &Env) {
        match event {
            druid::Event::MouseDown(e) => {
                ctx.set_active(true);
                self.set(e.pos, data);
            }
            druid::Event::MouseUp(_) => {
                ctx.set_active(false);
            }
            druid::Event::MouseMove(e) => {
                ctx.set_cursor(&Cursor::OpenHand);
                if ctx.is_active() {
                    self.set(e.pos, data);
                }
            }
            _ => ()
        }
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &druid::LifeCycle, _data: &Color, _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Color, _data: &Color, _env: &Env) {}
}


fn draw(width: usize, height: usize, get_px: impl Fn(usize, usize) -> [u8; 4]) -> Vec<u8> {
    let mut buf = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            let [r, g, b, a] = get_px(x, y);
            buf[ix] = r;
            buf[ix + 1] = g;
            buf[ix + 2] = b;
            buf[ix + 3] = a;
        }
    }
    buf
}
