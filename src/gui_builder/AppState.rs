use core::fmt;
use std::fmt::Display;
use std::path::Path;

use druid::{Data, Lens};
use im::Vector;
use super::error_correcting::ErrorCorrecting;
use super::decode_encode::{AlgorithmType, Decode, Encode};

/// current state of the application
#[derive(Clone, Data, Lens, Debug)]
pub struct AppState {
    pub file_path: String, //path to the file that gets converted
    pub save_path: String, //path to the place where the converted file gets saved
    pub error_correcting: ErrorCorrecting, //stores the selected error_correcting algorithm
    pub algorithm_type: AlgorithmType,
    pub decode_algorithm: Option<Decode>, //stores the selected decode algorithm
    pub encode_algorithm: Option<Encode>, //stores the selected encode algorithm
    pub calculating: f64, // The progress of the conversion from 0 to 1.
    pub calculating_msg: String, // Message to display current state of conversion
    pub error_msg: String, //stores error messages
    pub custom_msg_buf: String, //stores the current typed custom message for the FASTA header

    // Header Options
    pub header_file_name: bool, // Wether the name of the file should be shown.
    pub header_file_ext: bool, // Wether the file extension should be shown.
    pub header_file_size: bool, // ether the original size of the file should be shown.
    pub header_used_algorithm: bool, // Wether the used algorithm should be shown.
    pub header_used_error_correcting: bool, // Wether the used error correcting Code should be shown.
    pub header_custom_messages: Vector<String>, // All custom messages by the user

    pub debugging: bool, // Debugging state.
}

impl AppState {
    /// return the header of the FASTA file
    pub fn get_header(&self) -> Result<String, String> {
        let mut options: Vec<String> = Vec::new(); // Stores all selected header options
        let file_path = Path::new(&self.file_path);

        // Checking for activated settings
        if self.header_file_name {
            let file_name = match file_path.file_name() {
                Some(n) => n.to_str().unwrap().to_owned(),
                None => return Err("Error: selected file path terminates".to_string()),
            };
            options.push(file_name)
        }
        if self.header_file_ext {
            match file_path.extension() {
                Some(n) => options.push(n.to_str().unwrap().to_owned()),
                None => (),
            }
        }
        if self.header_file_size {
            let size = match file_path.metadata() {
                Ok(n) => n.len(),
                Err(e) => return Err(format!("Error while calling metadata: {}", e).to_string())
            };
            options.push(size.to_string());
        }
        if self.header_used_algorithm {
            let current_algorithm = match self.algorithm_type {
                AlgorithmType::Decode => {
                    match &self.decode_algorithm {
                        Some(n) => {
                            println!("{}", n.to_string());
                            n.to_string()
                        },
                        None => "None".to_string(),
                    }
                },
                AlgorithmType::Encode => {
                    match &self.encode_algorithm {
                        Some(n) => n.to_string(),
                        None => "None".to_string(),
                    }
                },
                AlgorithmType::None => "None".to_string(),
            };

            options.push(current_algorithm)
        }
        if self.header_used_error_correcting {
            options.push(self.error_correcting.to_string())
        }
        for msg in &self.header_custom_messages {
            options.push(msg.to_owned())
        }

        let mut output = ">".to_string(); // Start of header line
        let _ = options.iter().map(|option| output.push_str(&format!("{}|", option))).collect::<()>(); // Append all selected options to the header

        Ok(output)
    }

    /// Adds a custom messages for the FASTA header created by the user
    ///
    /// clears the custom messages buffer after adding the custom message
    pub fn add_custom_msg(&mut self) {
        self.header_custom_messages.push_back(self.custom_msg_buf.clone());
        self.custom_msg_buf.clear()
    }
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
            Calculating: {}\n
            Calculating_msg: {}\n
            Error_msg: {}\n
            Header: {}\n
            Debugging: {}\n",
            self.file_path,
            self.save_path,
            self.error_correcting,
            self.algorithm_type,
            self.decode_algorithm,
            self.encode_algorithm,
            self.calculating,
            self.calculating_msg,
            self.error_msg,
            self.get_header().unwrap_or("Error in get_header".to_string()),
            self.debugging,
        )
    }
}