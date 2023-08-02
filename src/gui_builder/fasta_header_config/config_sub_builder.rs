/*
contains a builder function for the root widget of the FASTA-Header-Configurator sub window
*/

use druid::{Widget, Env, WidgetExt, EventCtx};
use druid::widget::{Label, Flex, Either, Checkbox, LensWrap};


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
    root_widget
    )
}

fn fasta_customizer_builder() -> impl Widget<AppState> {
    let cb_file_name = Checkbox::new("file name")
        .lens(AppState::header_file_name);
    let cb_file_ext = Checkbox::new("file extension")
        .lens(AppState::header_file_ext);
    let cb_file_size = Checkbox::new("file size")
        .lens(AppState::header_file_size);
    let cb_selected_algorithm = Checkbox::new("selected algorithm")
        .lens(AppState::header_used_algorithm);
    let cb_selected_error_correcting = Checkbox::new("selected error correcting code")
        .lens(AppState::header_used_error_correcting);

    Flex::column()
        .with_child(cb_file_name)
        .with_child(cb_file_ext)
        .with_child(cb_file_size)
        .with_child(cb_selected_algorithm)
        .with_child(cb_selected_error_correcting)
}