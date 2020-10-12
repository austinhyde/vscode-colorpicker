use druid::{Cursor, Data, widget::Controller, widget::ControllerHost};
use druid::widget::prelude::*;

pub struct WithCursor(&'static Cursor);
impl<T, W: Widget<T>> Controller<T, W> for WithCursor {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::MouseMove(_) = event {
            ctx.set_cursor(self.0);
        }
        child.event(ctx, event, data, env);
    }
}

pub struct OnDataChange<T> {
    action: Box<dyn Fn(&T)>,
}
impl<T: Data> OnDataChange<T> {
    pub fn new(action: impl Fn(&T) + 'static) -> Self {
        Self{action: Box::new(action)}
    }
}
impl<T: Data, W: Widget<T>> Controller<T, W> for OnDataChange<T> {
    fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if !old_data.same(data) {
            (self.action)(data);
        }
        child.update(ctx, old_data, data, env);
    }
}

pub trait MoreWidgetExt<T: Data>: Widget<T> + Sized + 'static {
    fn on_data_change(self, action: impl Fn(&T) + 'static) -> ControllerHost<Self, OnDataChange<T>> {
        ControllerHost::new(self, OnDataChange::new(action))
    }
    fn with_cursor(self, cursor: &'static Cursor) -> ControllerHost<Self, WithCursor> {
        ControllerHost::new(self, WithCursor(cursor))
    }
}
impl<T: Data, W: Widget<T> + 'static> MoreWidgetExt<T> for W {}