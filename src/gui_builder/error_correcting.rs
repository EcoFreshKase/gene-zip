/*
contains a builder function for the selection of error correcting algorithms
*/
use std::fmt::Display;
use druid::{Widget, WidgetExt, Data, UnitPoint};
use druid::widget::{Label, Flex};
use druid_widget_nursery::DropdownSelect;

use super::AppState::AppState;
use super::{HAMMING, DEFAULT_OPTION};
use super::decode_encode::Algorithm;

/// represents a specific error correcting algorithm
#[derive(Data, Clone, PartialEq, Debug)]
pub enum ErrorCorrecting {
    None,
    Hamming,
}

impl Display for ErrorCorrecting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCorrecting::Hamming => write!(f, "Hamming"),
            ErrorCorrecting::None => write!(f, "None"),
        }
    }
}

impl Algorithm for ErrorCorrecting {}

pub fn error_correcting_builder() -> impl Widget<AppState> {

    let error_correcting_label = Label::new("Error Correcting:");
    //(displayed text, ErrorCorrecting variant) , ...
    let error_correcting_options = vec![
        (DEFAULT_OPTION, ErrorCorrecting::None),
        (HAMMING, ErrorCorrecting::Hamming)
    ];

    let dropdown = DropdownSelect::new(error_correcting_options)
        .lens(AppState::error_correcting); //dropdown menu for the selectionf of error correcting algorithms

    Flex::row()
        .with_flex_child(error_correcting_label, 1.0)
        .with_flex_child(dropdown, 1.0)
        .align_vertical(UnitPoint::TOP_LEFT)
}