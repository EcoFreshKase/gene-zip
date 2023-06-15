use std::fmt::{Display};

/*
Enthält eine Builder-Funktion für die Auswahl des Error-Correcting Algorithmuses
*/
use druid::{Widget, WidgetExt, Data};
use druid::widget::{Label, Flex};
use druid_widget_nursery::DropdownSelect;

use super::AppState::AppState;
use super::{HAMMING, BORDER_COLOR, BORDER_WIDTH, DEFAULT_OPTION};

#[derive(Data, Clone, PartialEq, Debug)]
pub enum ErrorCorrecting { //Jede Art
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

pub fn error_correcting_builder() -> impl Widget<AppState> {

    let error_correcting_label = Label::new("Error-Correcting:");

    //Vec mit allen Fehlerkorrektur Verfahren
    //(Display Text, ErrorCorrecting Variant) , ...
    let error_correcting_options = vec![
        (DEFAULT_OPTION, ErrorCorrecting::None),
        (HAMMING, ErrorCorrecting::Hamming)
    ];

    let dropdown = DropdownSelect::new(error_correcting_options)
        .lens(AppState::error_correcting); //dropdown Menü zur Auswahl des Fehlerkorrektur Verfahrens

    Flex::row()
        .with_child(error_correcting_label)
        .with_flex_child(dropdown, 1.0)
        .expand_width()
        .align_left()
}