use std::fmt::Display;

use crate::color::Color;
use druid::kurbo::Circle;
use druid::piet::{ImageFormat, InterpolationMode};
use druid::{
    kurbo::Line,
    widget::{prelude::*, BackgroundBrush, Painter},
    FontDescriptor, FontFamily, Key, TextLayout,
};
use druid::{
    BoxConstraints, Cursor, Data, Env, Event, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, Point,
    Rect, RenderContext, Size, UpdateCtx, Widget,
};

use crate::shape_util::*;

pub struct SatValuePicker {
    size: Size,
}

impl SatValuePicker {
    pub fn new() -> Self {
        Self {
            size: Size::new(0.0, 0.0),
        }
    }

    fn set(&self, p: Point, c: &mut Color) {
        c.set_saturation((p.x.max(0.0).min(self.size.width) / self.size.width) as f32);
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

        ctx.draw_image(&image, self.size.to_rect(), InterpolationMode::Bilinear);
        ctx.stroke(
            self.size.to_rounded_rect(1.0),
            &druid::Color::BLACK.with_alpha(0.2),
            0.5,
        );

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
                if ctx.is_active() {
                    ctx.set_active(false);
                }
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
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Color, data: &Color, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint()
        }
    }
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
        ctx.stroke(
            Rect::from_origin_size(Point::ORIGIN, self.size).to_rounded_rect(1.0),
            &druid::Color::BLACK.with_alpha(0.2),
            0.5,
        );

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
                ctx.set_cursor(&Cursor::ClosedHand);
                self.set(e.pos, data);
            }
            druid::Event::MouseUp(_) => {
                ctx.set_active(false);
                ctx.set_cursor(&Cursor::OpenHand);
            }
            druid::Event::MouseMove(e) => {
                if ctx.is_active() {
                    ctx.set_cursor(&Cursor::ClosedHand);
                    self.set(e.pos, data);
                } else {
                    ctx.set_cursor(&Cursor::OpenHand);
                }
            }
            _ => ()
        }
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &druid::LifeCycle, _data: &Color, _env: &Env) {}
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Color, data: &Color, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint()
        }
    }
}

pub struct AlphaPicker {
    size: Size,
}

impl AlphaPicker {
    pub fn new() -> Self {
        Self {
            size: Size::new(0.0, 0.0),
        }
    }

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
        ctx.stroke(
            Rect::from_origin_size(Point::ORIGIN, self.size).to_rounded_rect(1.0),
            &druid::Color::BLACK.with_alpha(0.2),
            0.5,
        );

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
                ctx.set_cursor(&Cursor::ClosedHand);
                self.set(e.pos, data);
            }
            druid::Event::MouseUp(_) => {
                ctx.set_active(false);
                ctx.set_cursor(&Cursor::OpenHand);
            }
            druid::Event::MouseMove(e) => {
                if ctx.is_active() {
                    ctx.set_cursor(&Cursor::ClosedHand);
                    self.set(e.pos, data);
                } else {
                    ctx.set_cursor(&Cursor::OpenHand);
                }
            }
            _ => ()
        }
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &druid::LifeCycle, _data: &Color, _env: &Env) {}
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Color, data: &Color, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint()
        }
    }
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


pub struct ToggleButton<T: Display> {
    variant: T,
    layout: TextLayout<String>,
    is_first: bool,
    is_last: bool,
}

impl<T: Data + Display + PartialEq> ToggleButton<T> {
    pub fn new(variant: T, is_first: bool, is_last: bool) -> ToggleButton<T> {
        ToggleButton {
            variant,
            layout: TextLayout::new(),
            is_first,
            is_last,
        }
    }

    pub fn is_active(&self, data: &T) -> bool {
        *data == self.variant
    }
}

pub const TOGGLE_ACTIVE_BG: Key<druid::Color> = Key::new("togglebutton.active.bg");
pub const TOGGLE_ACTIVE_FG: Key<druid::Color> = Key::new("togglebutton.active.fg");
pub const TOGGLE_INACTIVE_BG: Key<druid::Color> = Key::new("togglebutton.inactive.bg");
pub const TOGGLE_INACTIVE_FG: Key<druid::Color> = Key::new("togglebutton.inactive.fg");
pub const TOGGLE_BORDER: Key<druid::Color> = Key::new("togglebutton.border");

impl<T: Data + PartialEq + Display + std::fmt::Debug> Widget<T> for ToggleButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        *data = self.variant.clone();
                    }
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if matches!(event, LifeCycle::WidgetAdded) {
            self.layout.set_text(self.variant.to_string());
            self.layout
                .set_font(FontDescriptor::new(FontFamily::SYSTEM_UI));
            self.layout.set_text_size(9.0);
            if self.is_active(data) {
                self.layout.set_text_color(env.get(TOGGLE_ACTIVE_FG));
            } else {
                self.layout.set_text_color(env.get(TOGGLE_INACTIVE_FG));
            }
            self.layout.rebuild_if_needed(ctx.text(), env)
        }
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if !old_data.same(data) {
            if self.is_active(data) {
                self.layout.set_text_color(env.get(TOGGLE_ACTIVE_FG));
            } else {
                self.layout.set_text_color(env.get(TOGGLE_INACTIVE_FG));
            }
            self.layout.rebuild_if_needed(ctx.text(), env);
            ctx.request_paint();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        self.layout.set_wrap_width(bc.max().width);
        self.layout.rebuild_if_needed(ctx.text(), env);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();

        // Check if data enum matches our variant
        let bg = if self.is_active(data) {
            env.get(TOGGLE_ACTIVE_BG)
        } else {
            env.get(TOGGLE_INACTIVE_BG)
        };
        ctx.fill(Rect::new(0.0, 0.0, size.width, size.height), &bg);

        let border = env.get(TOGGLE_BORDER);
        if !self.is_first {
            ctx.stroke(Line::new((0.0, 0.0), (0.0, size.height)), &border, 1.0);
        }
        if !self.is_last {
            ctx.stroke(
                Line::new((size.width, 0.0), (size.width, size.height)),
                &border,
                1.0,
            );
        }

        if !self.is_active(data) {
            ctx.stroke(Line::new((0.0, 0.5), (size.width, 0.5)), &border, 1.0);
        }

        // Paint the text label
        let offset = (size.to_vec2() - self.layout.size().to_vec2()) / 2.0;
        self.layout.draw(ctx, offset.to_point());
    }
}
