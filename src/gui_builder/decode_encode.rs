/*
Enthält eine Builder Funktion für die Auswahl für Decoding und Encoding
von Dateien und die Auswahl für den genauene Encoding/Decoding Algorithmus
*/

use std::fmt::Display;

use druid::{Widget, Data, WidgetExt, UnitPoint};
use druid::widget::{RadioGroup, Flex, Label, LensWrap, Either, DisabledIf, Container, SizedBox};

use super::AppState::AppState;
use super::{BORDER_COLOR, BORDER_WIDTH};

#[allow(dead_code)]
#[derive(Data, Clone, PartialEq, Debug, Copy)]
pub enum AlgorithmType { //Ob ein Decode oder Encode Algorithmus genutzt werden soll
    Decode,
    Encode,
    None, //kein ausgewählter Algorithmus
}

#[allow(dead_code)]
#[derive(Data, Clone, PartialEq, Debug)]
pub enum Decode { //stellt einen Decode-Algorithmus dar
    EasyDecode
}

#[allow(dead_code)]
#[derive(Data, Clone, PartialEq, Debug)]
pub enum Encode { //stellt einen Encode-Algorithmus dar
    EasyEncode
}

impl Display for AlgorithmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmType::Decode => write!(f, "Decode"),
            AlgorithmType::Encode => write!(f, "Encode"),
            AlgorithmType::None => write!(f, "None"),
        }
    }
}

impl Display for Decode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decode::EasyDecode => write!(f, "Easy Encode"),
        }
    }
}

impl Display for Encode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Encode::EasyEncode => write!(f, "Easy Encode"),
        }
    }
}

pub fn builder() -> impl Widget<AppState> {
    let radio_group = LensWrap::new(
        RadioGroup::column(vec![
            ("Decode", AlgorithmType::Decode),
            ("Encode", AlgorithmType::Encode),
        ]),
        AppState::algorithm_type
        );

    let algorithm_choose = Either::new( //Auswahl des Algorithmuses
        |data: &AppState, _env| {
            match &data.algorithm_type {
                AlgorithmType::Decode => true,
                AlgorithmType::Encode => false,
                AlgorithmType::None => { //Nur bei fatalem Fehler //fail-safe
                    println!("Bei Erstellung des Algorithmen-Auswahl-Widgets ist data.algorithm Algorithm::None");
                    true
                },
            }
        },
        decode_builder(),
        encode_builder(),
    );
    let algorithm_choose_control: DisabledIf<AppState, Either<AppState>> = DisabledIf::new( //Auswahl des Algorthmuses nur möglich wenn Encode/Decode gewählt wurde
        algorithm_choose,
        |data: &AppState, _env| {
            match data.algorithm_type {
                AlgorithmType::None => true,
                _ => false,
            }
        }
    );
    Flex::column()
        .with_child(radio_group)
        .with_child(algorithm_choose_control)
        .center()
}

fn decode_builder() -> impl Widget<AppState> {
    let decode_label = Label::new("Decoder:");

    let options = vec![
        //("easy decode", Algorithm::Decode(Decode::EasyDecode)),
        ("easy decode", Some(Decode::EasyDecode)),
    ];

    let decode_dropdown = RadioGroup::column(options)
        .lens(AppState::decode_algorithm);

    Flex::row()
        .with_child(decode_label)
        .with_child(decode_dropdown)
}

fn encode_builder() -> impl Widget<AppState> {
    let encode_label = Label::new("Encoder:");

    let options = vec![
        ("easy encode", Some(Encode::EasyEncode)),
    ];

    let encode_dropdown = RadioGroup::column(options)
        .lens(AppState::encode_algorithm);

    Flex::row()
        .with_child(encode_label)
        .with_child(encode_dropdown)
}