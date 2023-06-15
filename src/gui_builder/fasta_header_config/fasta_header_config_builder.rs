/*
Enthält eine Builder-Funktion für die Konfiguration des fasta-headers
*/
use druid::{Widget, Data, Lens, WindowConfig, WidgetExt, UnitPoint};
use druid::widget::{Flex, Label, Button};

use crate::gui_builder::{BORDER_COLOR, BORDER_WIDTH};

use super::config_sub_builder::sub_window_builder;
use super::super::AppState::AppState;

#[derive(Data, Lens, Clone)]
struct TestData {
    string: &'static str,
}

pub fn config_builder() -> impl Widget<AppState> {
    let config_label = Label::new("fasta-header configuration:");
    let config_button = Button::new("config header")//Button neues Fenster öffnen
        .on_click(|ctx, data: &mut AppState, env| {
            let config_window = sub_window_builder();

            ctx.new_sub_window( //neues Fenster öffnen
                WindowConfig::default(), //Einstellungen für das Fenster
                config_window,
                data.clone(), 
                env.clone());
            });

    Flex::row()
        .with_child(config_label)
        .with_child(config_button)
        .align_left()
        .align_vertical(UnitPoint::TOP)
}