/*
Enthält eine Builder-Funktion, die ein root-widget für das Sub-Window zur Fasta-Header
personalisierung enthält.
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