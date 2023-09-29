/*
contains a builder function for the root widget of the FASTA-Header-Configurator sub window
*/

use druid::{Widget, Env, WidgetExt, Lens};
use druid::widget::{Label, Flex, Either, Checkbox, TextBox, Button, LabelText, LineBreaking};

use crate::gui_builder::AppState::AppState;

use super::FASTA_WINDOW_SIZE;

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

    // Shows the current header
    let current_header = Label::dynamic(|data: &AppState, _env: &Env| {
        let mut text = "Current Headline: \n".to_string();
        text.push_str( match &data.get_header() {
            Ok(n) => n.as_str(),
            Err(e) => e.as_str(),
        });
        text
    }).with_line_break_mode(LineBreaking::WordWrap)
    .padding((0.0, 20.0));

    Flex::column()
        .with_child(cb_container_builder())
        .with_child(custom_msg_builder())
        .with_child(current_header)
        .fix_size(FASTA_WINDOW_SIZE.0, FASTA_WINDOW_SIZE.1)
}

/// Creates the checkboxes to control the Header
fn cb_container_builder() -> impl Widget<AppState> {
    let cb_file_name = cb_builder("file name", AppState::header_file_name);
    let cb_file_ext = cb_builder("file extension", AppState::header_file_ext);
    let cb_file_size = cb_builder("file size", AppState::header_file_size);
    let cb_selected_algorithm = cb_builder("selected algorithm", AppState::header_used_algorithm);
    let cb_selected_error_correcting = cb_builder("selected error correcting code", AppState::header_used_error_correcting);

    let padding = (599.0 * 0.3, 20.0, 5.0, 20.0);

    Flex::column()
        .with_child(cb_file_name)
        .with_child(cb_file_ext)
        .with_child(cb_file_size)
        .with_child(cb_selected_algorithm)
        .with_child(cb_selected_error_correcting)
        .padding(padding)
        .center()
}

/// Creates a Checkbox with the given Text and wraps the checkbox to the given Data
fn cb_builder(text: impl Into<LabelText<bool>>, lens_data: impl Lens<AppState, bool>) -> impl Widget<AppState> {

    Checkbox::new(text)
        .align_left()
        .lens(lens_data)
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

    let padding = (0.0, 20.0);

    Flex::row()
        .with_child(text_box)
        .with_child(add_button)
        .with_child(remove_button)
        .padding(padding)
        .center()
}