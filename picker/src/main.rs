use druid::{FontDescriptor, FontFamily, widget::{BackgroundBrush, Flex, Label, Painter}};
use druid::{AppLauncher, Data, Lens, PlatformError, RenderContext, Widget, WidgetExt, WindowDesc};
use structopt::StructOpt;

mod color;
use color::Color;

mod pickers;
use pickers::*;

#[derive(StructOpt, Debug, Clone)]
struct Args {
    #[structopt(default_value = "#123456")]
    color: Color,
}

#[derive(Clone, Data, Lens)]
struct PickerState {
    initial_color: Color,
    current_color: Color,
}

impl PickerState {
    fn new(args: &Args) -> Self {
        Self {
            initial_color: args.color.clone(),
            current_color: args.color.clone(),
        }
    }
}

#[derive(Clone)]
struct Sizing {
    padding: f64,
    picker_size: f64,
    slider_size: f64,
    current_swatch_size: f64,
    initial_swatch_size: f64,
}
impl Sizing {
    fn window_size(&self) -> (f64, f64) {
        (
            self.window_width(),
            self.window_height(),
        )
    }
    fn window_width(&self) -> f64 {
        self.padding*4.0 + self.picker_size + self.slider_size*2.0
    }
    fn window_height(&self) -> f64 {
        self.current_swatch_size + self.initial_swatch_size + self.padding*2.0 + self.picker_size
    }
}

fn main() -> Result<(), PlatformError> {
    let args = Args::from_args();
    let data = PickerState::new(&args);

    let sizing = Sizing{
        padding: 10.0,
        picker_size: 256.0,
        slider_size: 25.0,
        current_swatch_size: 50.0,
        initial_swatch_size: 30.0,
    };

    let main_window = WindowDesc::new(build_root(sizing.clone()))
        .window_size(sizing.window_size())
        .show_titlebar(false);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn build_root(sizing: Sizing) -> impl Fn() -> Flex<PickerState> {
    move || {
        Flex::column()
            .must_fill_main_axis(true)
            .with_child(
                swatch()
                    .background(pickers::checkered_bgbrush())
                    .fix_size(sizing.window_width(), sizing.current_swatch_size)
                    .lens(PickerState::current_color),
            )
            .with_child(
                swatch()
                    .background(pickers::checkered_bgbrush())
                    .fix_size(sizing.window_width(), sizing.initial_swatch_size)
                    .lens(PickerState::initial_color),
            )
            .with_child(
                hsla_picker(&sizing)
                    .lens(PickerState::current_color),
            )
    }
}
fn swatch() -> impl Widget<Color> {
    let label = Label::dynamic(|c: &Color, _| c.hex())
        // Druid 0.6.0
        // .with_font("Courier New".to_string());
        // Druid master
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE));
    let painter = Painter::new(|ctx, data: &Color, _env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &data.to_druid())
    });
    label.center()
        .background(pickers::checkered_bgbrush())
        .background(BackgroundBrush::Painter(painter))
}

fn hsla_picker(sizing: &Sizing) -> impl Widget<Color> {
    Flex::row()
        .with_child(SatLightPicker::new().fix_size(sizing.picker_size, sizing.picker_size))
        .with_spacer(sizing.padding)
        .with_child(HuePicker::new().fix_size(sizing.slider_size, sizing.picker_size))
        .with_spacer(sizing.padding)
        .with_child(AlphaPicker::new().fix_size(sizing.slider_size, sizing.picker_size).background(pickers::checkered_bgbrush()))
        .padding(sizing.padding)
}
