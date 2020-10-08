use std::{fmt::Display, str::FromStr};

use druid::{AppLauncher, Data, FontDescriptor, FontFamily, Lens, PlatformError, RenderContext, Widget, WidgetExt, WindowDesc};
use druid::widget::{BackgroundBrush, Flex, Label, Painter};
use structopt::StructOpt;

mod color;
use color::Color;

mod pickers;
use pickers::*;

#[derive(Debug, Clone)]
enum Position {
    Under,
    Over,
}
impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "under" => Ok(Position::Under),
            "over" => Ok(Position::Over),
            s => Err(format!("Invalid value: {}", s)),
        }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Under => "under",
            Self::Over => "over",
        })
    }
}

#[derive(StructOpt, Debug, Clone)]
struct Args {
    #[structopt(default_value = "#123456")]
    color: Color,

    #[structopt(short, default_value = "100.0")]
    x: f64,

    #[structopt(short, default_value = "100.0")]
    y: f64,

    #[structopt(long, default_value = "under")]
    position: Position,

    #[structopt(long)]
    font: Option<String>,
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
        picker_size: 150.0,
        slider_size: 25.0,
        current_swatch_size: 50.0,
        initial_swatch_size: 30.0,
    };

    let main_window = WindowDesc::new(build_root(args.clone(), sizing.clone()))
        .window_size(sizing.window_size())
        .set_position(druid::kurbo::Point::new(args.x, args.y))
        .resizable(false)
        .show_titlebar(false);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn build_root(args: Args, sizing: Sizing) -> impl Fn() -> Flex<PickerState> {
    move || {
        let curr_swatch = swatch(&args)
                        .background(pickers::checkered_bgbrush())
                        .fix_size(sizing.window_width(), sizing.current_swatch_size)
                        .lens(PickerState::current_color);
        let init_swatch = swatch(&args)
                        .background(pickers::checkered_bgbrush())
                        .fix_size(sizing.window_width(), sizing.initial_swatch_size)
                        .lens(PickerState::initial_color);
        let picker = hsla_picker(&sizing)
                        .lens(PickerState::current_color);

        match args.position {
            Position::Under =>
                Flex::column()
                    .must_fill_main_axis(true)
                    .with_child(curr_swatch)
                    .with_child(init_swatch)
                    .with_child(picker),

            Position::Over =>
                Flex::column()
                    .must_fill_main_axis(true)
                    .with_child(picker)
                    .with_child(init_swatch)
                    .with_child(curr_swatch)
        }
    }
}
fn swatch(args: &Args) -> impl Widget<Color> {
    let label = Label::dynamic(|c: &Color, _| c.hex())
        // Druid 0.6.0
        // .with_font("Courier New".to_string());
        // Druid master
        .with_font(druid::FontDescriptor::new(args.font.clone().map_or(FontFamily::MONOSPACE, FontFamily::new_unchecked)));
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
