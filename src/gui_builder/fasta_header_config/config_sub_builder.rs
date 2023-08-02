/*
contains a builder function for the root widget of the FASTA-Header-Configurator sub window
*/

use druid::{Widget, Env, WidgetExt};
use druid::widget::{Label, Flex, Either, Checkbox, TextBox, Button};


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

    // Shows the current header
    let current_header = Label::dynamic(|data: &AppState, _env: &Env| {
        let mut text = "Current Headline: \n".to_string();
        text.push_str( match &data.get_header() {
            Ok(n) => n.as_str(),
            Err(e) => e.as_str(),
        });
        text
    });


    Flex::column()
        .with_child(cb_file_name)
        .with_child(cb_file_ext)
        .with_child(cb_file_size)
        .with_child(cb_selected_algorithm)
        .with_child(cb_selected_error_correcting)
        .with_child(custom_msg_builder())
        .with_child(current_header)
}

/// Returns a Widget containing a TextBox and Button to allow for custom messages created by the user to be used
/// for the FASTA Header
fn custom_msg_builder() -> impl Widget<AppState> {
    let text_box = TextBox::new()
        .with_line_wrapping(true)
        .lens(AppState::custom_msg_buf);
    let add_button = Button::new("Add message")
        .on_click(|_ctx, data: &mut AppState, _env| {
            data.add_custom_msg()
        });
    let remove_button = Button::new("Remove last message")
        .on_click(|_ctx, data: &mut AppState, _env| {
            data.header_custom_messages.pop_back();
        });

    Flex::row()
        .with_child(text_box)
        .with_child(add_button)
        .with_child(remove_button)
}