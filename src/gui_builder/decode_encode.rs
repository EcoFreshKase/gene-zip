/*
contains a builder function for the selection wether to decode or encode the file 
as well as the specific algorithm selection
*/

use std::fmt::{Display, Debug};

use druid::{Widget, Data, WidgetExt};
use druid::widget::{RadioGroup, Flex, Label, LensWrap, Either, DisabledIf};

use super::AppState::AppState;

/// Types implementing this Trait represent an Algorithm
pub trait Algorithm: Display + Debug {}

/// represents an the kind of an algorithm
#[allow(dead_code)]
#[derive(Data, Clone, PartialEq, Debug, Copy)]
pub enum AlgorithmType {
    Decode,
    Encode,
    None, //no algorithm
}

/// represents a specific Decode Algorithm
#[allow(dead_code)]
#[derive(Data, Clone, PartialEq, Debug)]
pub enum Decode {
    EasyDecode
}

/// represents a specific Encode Algorithm
#[allow(dead_code)]
#[derive(Data, Clone, PartialEq, Debug)]
pub enum Encode {
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
            Decode::EasyDecode => write!(f, "Easy Decode"),
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

impl Algorithm for Decode {}
impl Algorithm for Encode {}

pub fn builder() -> impl Widget<AppState> {
    let radio_group = LensWrap::new(
        RadioGroup::column(vec![
            ("Decode", AlgorithmType::Decode),
            ("Encode", AlgorithmType::Encode),
        ]),
        AppState::algorithm_type
        );

    let algorithm_choose = Either::new( //selection of the algorithm
        |data: &AppState, _env| {
            match &data.algorithm_type {
                AlgorithmType::Decode => true,
                AlgorithmType::Encode => false,
                AlgorithmType::None => {//fail-safe
                    println!("\"data.algorithm == Algorithm::None\" while creating algorithm_choose widget");
                    true
                },
            }
        },
        decode_builder(),
        encode_builder(),
    );
    let algorithm_choose_control: DisabledIf<AppState, Either<AppState>> = DisabledIf::new( //algorithm selection only possible if an algorithm type was selected
        algorithm_choose,
        |data: &AppState, _env| {
            matches!(data.algorithm_type, AlgorithmType::None)
        }
    );
    Flex::column()
        .with_child(radio_group)
        .with_child(algorithm_choose_control)
        .center()
}

fn decode_builder() -> impl Widget<AppState> {
    let decode_label = Label::new("Decoder:")
        .center();

    let options = vec![
        ("easy decode", Some(Decode::EasyDecode)),
    ];

    let decode_dropdown = RadioGroup::column(options)
        .padding(5.0)
        .lens(AppState::decode_algorithm);

    Flex::row()
        .with_child(decode_label)
        .with_child(decode_dropdown)
        .padding(5.0)
}

fn encode_builder() -> impl Widget<AppState> {
    let encode_label = Label::new("Encoder:")
        .center();

    let options = vec![
        ("easy encode", Some(Encode::EasyEncode)),
    ];

    let encode_choose = RadioGroup::column(options)
        .padding(5.0)
        .lens(AppState::encode_algorithm);

    Flex::row()
        .with_child(encode_label)
        .with_child(encode_choose)
        .padding(5.0)
}