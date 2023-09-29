/*
contains everything to create the GUI
*/
use druid::{Widget, Color, Event, WidgetExt};
use druid::widget::{Flex, Controller, Checkbox};

#[allow(non_snake_case)]
pub mod AppState;
pub mod file_path;
pub mod error_correcting;
pub mod fasta_header_config;
pub mod decode_encode;
pub mod start_button;
mod error_window;
mod loading_window;

pub use error_window::open_error;
pub use decode_encode::{AlgorithmType, Decode, Encode};

use crate::GLOBAL_UPDATE;

//constants for the UI creation
pub const BACKGROUND_COLOR: Color = Color::WHITE;
pub const TEXT_COLOR: Color = Color::BLACK;
pub const BORDER_COLOR: Color = Color::BLACK;
pub const BORDER_WIDTH: f64 = 3.0;
pub const DEFAULT_OPTION: &str = "no selection";
//Error correcting algorithm representations
pub const HAMMING: &str = "Hamming Code";

///returns a widget containing the whole UI
pub fn build_ui() -> impl Widget<AppState::AppState> {

    let debugging_checkbox = Checkbox::new("Debugging")
        .lens(AppState::AppState::debugging);

    Flex::column()
        .with_flex_child(file_path::file_path_builder(), 1.0)
        .with_flex_child(error_correcting::error_correcting_builder(), 1.0)
        .with_flex_child(Flex::row()
            .with_flex_child(fasta_header_config::config_builder(), 1.0)
            .with_flex_child(decode_encode::builder(), 1.0),
            1.0,
        )
        .with_flex_child(start_button::start_button_builder(), 1.0)
        //.with_child(debugging_checkbox)
}

struct MainController;

impl<T, W: Widget<T>> Controller<T,W> for MainController {
    fn event(&mut self, _child: &mut W, ctx: &mut druid::EventCtx, event: &druid::Event, _data: &mut T, _env: &druid::Env) {
        match event {
            Event::Command(cmd) if cmd.is(GLOBAL_UPDATE) => ctx.request_update(),
            _ => (),
        }
    }
}