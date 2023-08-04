/*
contains a builder function for the fasta-header configurator
*/
use druid::{Widget, WindowConfig, WidgetExt, UnitPoint};
use druid::widget::{Flex, Label, Button};

use super::config_sub_builder::sub_window_builder;
use super::super::AppState::AppState;

pub const FASTA_WINDOW_SIZE: (f64, f64) = (599.0 *1.0, 337.0 *1.0);

pub fn config_builder() -> impl Widget<AppState> {
    let config_label = Label::new("fasta-header configuration:");
    let config_button = Button::new("config header")//button to create sub window
        .on_click(|ctx, data: &mut AppState, env| {
            let config_window = sub_window_builder();

            let window_setting = WindowConfig::default() //sub window settings
            .resizable(false)
            .window_size(FASTA_WINDOW_SIZE);

            ctx.new_sub_window(
                window_setting, 
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