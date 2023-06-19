/*
contains a buidler function for the fasta-header configurator
*/
use druid::{Widget, Data, Lens, WindowConfig, WidgetExt, UnitPoint};
use druid::widget::{Flex, Label, Button};

use super::config_sub_builder::sub_window_builder;
use super::super::AppState::AppState;

#[derive(Data, Lens, Clone)]
struct TestData {
    string: &'static str,
}

pub fn config_builder() -> impl Widget<AppState> {
    let config_label = Label::new("fasta-header configuration:");
    let config_button = Button::new("config header")//button to create sub window
        .on_click(|ctx, data: &mut AppState, env| {
            let config_window = sub_window_builder();

            ctx.new_sub_window(
                WindowConfig::default(), //sub window settings
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