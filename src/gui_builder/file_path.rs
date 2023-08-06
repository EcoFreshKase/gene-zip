/*
contains a builder function for a widget that lets the user see the path of the current file
and the path to which the file gets saved
*/

use druid::{Widget, UnitPoint, Env};
use druid::widget::{Label, WidgetExt, TextBox, Flex, LineBreaking};
use druid::text::FontDescriptor;

use crate::gui_builder::AppState::AppState;

pub fn file_path_builder() -> impl Widget<AppState> {
    let path_font =FontDescriptor::new(druid::piet::FontFamily::SYSTEM_UI)
        .with_size(13.5);

    let padding = (0.0, 3.0);

    let current_path_label = Label::new("Current Path:");
    let file_path_label = Label::new(|data: &AppState, _env: &Env| { //create file_path_label
        data.file_path.to_string()
    }).with_font(path_font.clone())
    .with_line_break_mode(LineBreaking::WordWrap)
    .align_left();
    let current_path_container = Flex::row() // Contains all Widgets showing the current selected path
        .with_child(current_path_label)
        .with_child(file_path_label)
        .padding(padding);

    let save_path_label = Label::new("Save Path:");
    let file_name_text_box = TextBox::multiline() //field to input the name of the file to save
        .with_font(path_font)
        .with_line_wrapping(true)
        .lens(AppState::save_path);
    let save_path_container = Flex::row()
        .with_child(save_path_label)
        .with_child(file_name_text_box)
        .padding(padding);

    Flex::column()
        .with_child(current_path_container)
        .with_child(save_path_container)
        .align_vertical(UnitPoint::TOP_LEFT)
        .align_left()
}