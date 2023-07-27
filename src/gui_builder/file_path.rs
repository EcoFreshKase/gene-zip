/*
contains a builder function for a widget that lets the user see the path of the current file
and the path to which the file gets saved
*/

use druid::{Widget,Insets, UnitPoint, Env};
use druid::widget::{Label, WidgetExt, TextBox, Padding, Flex, LineBreaking};
use druid::text::FontDescriptor;

use crate::gui_builder::AppState::AppState;

pub fn file_path_builder() -> impl Widget<AppState> {
    let path_font =FontDescriptor::new(druid::piet::FontFamily::SYSTEM_UI)
        .with_size(13.5);

    let file_text = Label::new("File-Path:");
    
    let file_path_label = Label::new(|data: &AppState, _env: &Env| { //create file_path_label
        format!("{}", data.file_path)
    }).with_font(path_font.clone())
    .with_line_break_mode(LineBreaking::WordWrap)
    .align_left();
    let file_name_text_box = TextBox::multiline() //field to input the name of the file to save
        .with_font(path_font)
        .with_line_wrapping(true)
        .lens(AppState::save_path);
    let file_path_container = Padding::new(Insets { //padding widget
        x0: 10.0,
        y0: 0.0,
        x1: 0.0,
        y1: 0.0,
    }, Flex::column()
        .with_child(file_path_label)
        .with_child(file_name_text_box));

    Flex::row()
        .with_child(file_text)
        .with_child(file_path_container)
        .align_vertical(UnitPoint::TOP_LEFT)
        .align_left()
}