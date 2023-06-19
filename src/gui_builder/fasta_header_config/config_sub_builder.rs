/*
contains a builder function for the root widget of the FASTA-Header-Configurator sub window
*/

use druid::Widget;
use druid::widget::{Label, Flex};

use crate::gui_builder::AppState::AppState;

pub fn sub_window_builder() -> impl Widget<AppState> {
    Flex::row()
        .with_child(Label::dynamic(|data: &AppState, _env| {
            format!("{}", data)
        }))
}