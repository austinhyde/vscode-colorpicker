use druid::{Point, Rect, Size, kurbo::Circle};


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

