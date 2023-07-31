/*
contains a builder function for the root widget of the FASTA-Header-Configurator sub window
*/

use druid::{Widget, Env};
use druid::widget::{Label, Flex, Either, Checkbox};


use crate::gui_builder::AppState::AppState;

pub fn sub_window_builder() -> impl Widget<AppState> {
    let root_widget = fasta_customizer_builder();

    Either::new(
        |data: &AppState, _env: &Env| {
            data.debugging
        },
    Flex::row()
        .with_child(Label::dynamic(|data: &AppState, _env| {
            format!("{}", data)
        })),
    root_widget,
    )
}

fn fasta_customizer_builder() -> impl Widget<AppState> {
    let root_widget = Flex::column();

    root_widget
}