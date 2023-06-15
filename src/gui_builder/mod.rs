/*
Enthält die gesamte Logik das Grafische Interface zu erzeugen
*/
use druid::{Widget, Color, WidgetExt};
use druid::widget::{Flex, Split, SizedBox};

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


//Konstanten für die UI Erstellung
pub const BACKGROUND_COLOR: Color = Color::WHITE;
pub const TEXT_COLOR: Color = Color::BLACK;
pub const BORDER_COLOR: Color = Color::BLACK;
pub const BORDER_WIDTH: f64 = 3.0;
pub const DEFAULT_OPTION: &str = "Keine Auswahl"; //Wenn keine Auswahl getroffen wurde
//Fehlerkorrektur-Algorithmen
pub const HAMMING: &'static str = "Hamming Code";

pub fn build_ui() -> impl Widget<AppState::AppState> {
    /*
    Returns a Widget that represents the whole app
    */

    Flex::column() //Enthält gesamte Benutzeroberfläche
        .with_flex_child(file_path::file_path_builder(), 1.0)
        .with_flex_child(error_correcting::error_correcting_builder(), 1.0)
        .with_flex_child(Flex::row()
            .with_flex_child(fasta_header_config::config_builder(), 1.0)
            .with_flex_child(decode_encode::builder(), 1.0),
            1.0,
        )
        .with_flex_child(start_button::start_button_builder(), 1.0)
}