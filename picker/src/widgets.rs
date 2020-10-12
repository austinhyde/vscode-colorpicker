use crate::color::Color;
use druid::{BoxConstraints, Cursor, Data, Env, Event, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget, widget::BackgroundBrush, widget::Controller, widget::ControllerHost, widget::Painter};
use druid::kurbo::Circle;
use druid::piet::{ImageFormat, InterpolationMode};

pub struct SatValuePicker {
    size: Size,
}

impl SatValuePicker {
    pub fn new() -> Self { Self { size: Size::new(0.0, 0.0) } }

    fn set(&self, p: Point, c: &mut Color) {
        // x is [0..1] saturation
        c.set_saturation((p.x.max(0.0).min(self.size.width) / self.size.width) as f32);
        // y is [1..0] value
        c.set_value((1.0 - p.y.max(0.0).min(self.size.height) / self.size.height) as f32);
    }
}

impl Widget<Color> for SatValuePicker {
    fn paint(&mut self, ctx: &mut PaintCtx, data: &Color, _env: &Env) {
        self.size = ctx.size();
        let width = self.size.width.floor() as usize;
        let height = self.size.height.floor() as usize;

        let buf = draw(width, height, |x, y| {
            let sat = x as f32 / width as f32;
            let value = 1.0 - y as f32 / width as f32;
            Color::from_hsva_f32(data.hue(), sat, value, 1.0).pixel()
        });

        let image = ctx
            .make_image(width, height, &buf, ImageFormat::RgbaSeparate)
            .unwrap();

        ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, self.size),
            InterpolationMode::Bilinear,
        );
        ctx.stroke(Rect::from_origin_size(Point::ORIGIN, self.size).to_rounded_rect(1.0), &druid::Color::BLACK.with_alpha(0.2), 0.5);

        let x = data.saturation() as f64 * width as f64;
        let y = (1.0 - data.value() as f64) * height as f64;
        let size = 4.5;
        let stroke = 2.0;
        let inset = 1.0;
        let circle = Circle::new(Point::new(x, y), size)
            .shrink(stroke/2.0)
            .clamp(
                Rect::new(0.0, 0.0, width as f64, height as f64)
                .shrink(Size::new(inset, inset))
                .shrink(Size::new(stroke/2.0, stroke/2.0))
            );
        let shadow_circle = circle.translate(0.0, 1.0);
        ctx.stroke(shadow_circle, &druid::Color::BLACK.with_alpha(0.2), stroke);
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
            Color::from_hsva_f32(hue, data.saturation(), data.value(), 1.0).pixel()
        });

        let image = ctx
            .make_image(width, height, &buf, ImageFormat::RgbaSeparate)
            .unwrap();

        ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, self.size),
            InterpolationMode::Bilinear,
        );
        ctx.stroke(Rect::from_origin_size(Point::ORIGIN, self.size).to_rounded_rect(1.0), &druid::Color::BLACK.with_alpha(0.2), 0.5);

        let y = data.hue() as f64 * height as f64;
        let size = 5.0;
        let inset = 1.0;
        let stroke = 2.0;

        let rect = Rect::new(0.0, y, width as f64, y + size)
            .translate(0.0, -size/2.0)
            .shrink(Size::new(inset, 0.0))
            .shrink(Size::new(stroke/2.0, stroke/2.0))
            .clamp(
                Rect::new(0.0, 0.0, width as f64, height as f64)
                .shrink(Size::new(stroke/2.0, stroke/2.0))
            );
            let rect_shadow = rect.translate(0.0, 0.5);
        ctx.stroke(rect_shadow.to_rounded_rect(0.5), &druid::Color::BLACK.with_alpha(0.2), stroke);
        ctx.stroke(rect.to_rounded_rect(0.5), &druid::Color::WHITE, stroke);
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
            Color::from_hsva_f32(data.hue(), data.saturation(), data.value(), alpha).pixel()
        });

        let image = ctx
            .make_image(width, height, &buf, ImageFormat::RgbaSeparate)
            .unwrap();

        ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, self.size),
            InterpolationMode::Bilinear,
        );
        ctx.stroke(Rect::from_origin_size(Point::ORIGIN, self.size).to_rounded_rect(1.0), &druid::Color::BLACK.with_alpha(0.2), 0.5);

        let y = (1.0 - data.alpha()) as f64 * height as f64;
        let size = 5.0;
        let inset = 1.0;
        let stroke = 2.0;

        let rect = Rect::new(0.0, y, width as f64, y + size)
            .translate(0.0, -size/2.0)
            .shrink(Size::new(inset, 0.0))
            .shrink(Size::new(stroke/2.0, stroke/2.0))
            .clamp(
                Rect::new(0.0, 0.0, width as f64, height as f64)
                .shrink(Size::new(stroke/2.0, stroke/2.0))
            );
        let rect_shadow = rect.translate(0.0, 0.5);
        ctx.stroke(rect_shadow.to_rounded_rect(0.5), &druid::Color::BLACK.with_alpha(0.2), stroke);
        ctx.stroke(rect.to_rounded_rect(0.5), &druid::Color::WHITE, stroke);
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


pub fn checkered_bgbrush<T>(checker_side: f64) -> BackgroundBrush<T> {
    BackgroundBrush::Painter(Painter::new(move |ctx, _data, _env| {
        let size = ctx.size();
        let width = size.width as usize;
        let height = size.height as usize;
        ctx.fill(size.to_rect(), &druid::Color::WHITE);

        let checker_size = Size::new(checker_side as f64, checker_side as f64);
        let grey = druid::Color::grey(0.9);
        for x in (0..width).step_by(checker_side as usize*2) {
            for y in (0..height).step_by(checker_side as usize*2) {
                ctx.fill(Rect::from_origin_size(Point::new(x as f64 + checker_side, y as f64), checker_size), &grey);
                ctx.fill(Rect::from_origin_size(Point::new(x as f64, y as f64 + checker_side), checker_size), &grey);
            }
        }
    }))
}

pub struct WithCursor(&'static Cursor);
impl<T, W: Widget<T>> Controller<T, W> for WithCursor {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::MouseMove(_) = event {
            ctx.set_cursor(self.0);
        }
        child.event(ctx, event, data, env);
    }
}
pub trait WithCursorExt<T: Data>: Widget<T> + Sized + 'static {
    fn with_cursor(self, cursor: &'static Cursor) -> ControllerHost<Self, WithCursor> {
        ControllerHost::new(self, WithCursor(cursor))
    }
}
impl<T: Data, W: Widget<T> + 'static> WithCursorExt<T> for W {}

pub trait ShapeHelpExt where Self: Sized {
    type K;
    fn translate(&self, x: f64, y: f64) -> Self;
    fn shrink(&self, k: Self::K) -> Self;
    fn clamp(&self, bounds: Rect) -> Self;
}
impl ShapeHelpExt for Rect {
    type K = Size;
    fn translate(&self, x: f64, y: f64) -> Self {
        Rect::new(
            self.x0 + x,
            self.y0 + y,
            self.x1 + x,
            self.y1 + y
        )
    }

    fn shrink(&self, k: Size) -> Self {
        Rect::new(
            self.x0 + k.width,
            self.y0 + k.height,
            self.x1 - k.width,
            self.y1 - k.height,
        )
    }

    fn clamp(&self, bounds: Rect) -> Self {
        Rect::new(
            self.x0.max(bounds.x0).min(bounds.x1 - self.width()),
            self.y0.max(bounds.y0).min(bounds.y1 - self.height()),
            self.x1.max(bounds.x0 + self.width()).min(bounds.x1),
            self.y1.max(bounds.y0 + self.height()).min(bounds.y1),
        )
    }
}
impl ShapeHelpExt for Circle {
    type K = f64;

    fn translate(&self, x: f64, y: f64) -> Self {
        Circle::new(Point::new(self.center.x + x, self.center.y + y), self.radius)
    }

    fn shrink(&self, k: f64) -> Self {
        Circle::new(self.center, self.radius - k)
    }

    fn clamp(&self, bounds: Rect) -> Self {
        Circle::new(
            Point::new(
                self.center.x.max(bounds.x0 + self.radius).min(bounds.x1 - self.radius),
                self.center.y.max(bounds.y0 + self.radius).min(bounds.y1 - self.radius),
            ),
            self.radius,
        )
    }
}
