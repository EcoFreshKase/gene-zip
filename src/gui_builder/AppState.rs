use core::fmt;
use std::fmt::Display;

use druid::{Data, Lens};
use super::error_correcting::ErrorCorrecting;
use super::decode_encode::{AlgorithmType, Decode, Encode};

/// current state of the application
#[derive(Clone, Data, Lens, Debug)]
pub struct AppState {
    pub file_path: String, //path to the file that gets converted
    pub save_path: String, //path to the place where the converted file gets saved
    pub error_correcting: ErrorCorrecting, //stores the selectes error_correcting algorithm
    pub algorithm_type: AlgorithmType,
    pub decode_algorithm: Option<Decode>, //storers the selected decode algorithm
    pub encode_algorithm: Option<Encode>, //storers the selected encode algorithm
    pub calculating: bool, //whether a conversion is in progress
}

impl Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "File_Path: {}\n
            Save_Path: {}\n
            Error_Correcting: : {}\n
            Algorithm_Type: {}\n
            Decode_Algorithm: {:?}\n
            Encode_Algorithm: {:?}\n
            Calculating: {}\n",
            self.file_path,
            self.save_path,
            self.error_correcting,
            self.algorithm_type,
            self.decode_algorithm,
            self.encode_algorithm,
            self.calculating,
        )
    }
}