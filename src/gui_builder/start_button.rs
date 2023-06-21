/*
Enthält eine builder Funktion für den Button, mit dem die Konvertierung gestartet wird

contains a builder function for the button that starts the conversion
*/

use std::fmt::Display;
use std::path::Path;
use std::fs::{self, read};
use druid::{Widget, EventCtx, Env};
use druid::widget::{Button};
use super::AppState::AppState;
use super::error_correcting::ErrorCorrecting;
use super::{AlgorithmType, Decode, Encode};
use super::{open_error, loading_window::open_loading};
use crate::convert_utils::{easy_encode, easy_decode};
use crate::convert_utils::error_correcting::hamming_code::{hamming_decode, hamming_encode};
use crate::ERROR;

pub fn start_button_builder() -> impl Widget<AppState> {
    let button = Button::new("Convert")
        .on_click(|ctx: &mut EventCtx, data: &mut AppState, env: &Env| {
            if let AlgorithmType::None = data.algorithm_type{ //error message if no algorithm is selected
                open_error(ctx, data, env, "Choose an algorithm!");
                return;
            }
            if let None = data.decode_algorithm { //error message if no algorithm is selected
                if let None = data.encode_algorithm {
                    open_error(ctx, data, env, "Choose an algorithm!");
                    return;
                }
            }

            //conversion of the file
            data.calculating = true;
            open_loading(ctx, data, env);
            let mut error_msg = None; // stores error messages
            match data.algorithm_type {
                AlgorithmType::Encode => match encode_file(
                Path::new(&data.file_path),
                Path::new(&data.save_path), 
                data.encode_algorithm.clone().unwrap(),
                &data.error_correcting) {
                    Ok(_) => (),
                    Err(e) => error_msg = Some(e),
                },
                AlgorithmType::Decode => match decode_file(
                Path::new(&data.file_path),
                Path::new(&data.save_path), 
                data.decode_algorithm.clone().unwrap(),
                &data.error_correcting) {
                    Ok(_) => (),
                    Err(e) => error_msg = Some(e),
                },
                AlgorithmType::None => error_msg = Some("Choose an algorithm!".to_string()),
            }
            if let Some(_) = error_msg {
                ctx.submit_command(ERROR);
                open_error(ctx, data, env, error_msg.unwrap());
            }
            data.calculating = false;
        });
    button
}

///encodes a file at the given path with the given algorithm and saves it in the given path
/// 
/// returns an error if:
///     - metadata of the file couldn't been read
///     - the path is not a file
///     - the save_path already exists
///     - save_path couldn't be converted to a string
///     - an error occurred while encoding
///     - the name of the given file is invalid
///     - an error occurred while writing to a file
fn encode_file(file_path: &std::path::Path, save_path: &std::path::Path, algorithm: Encode, error_correcting_algorithm: &ErrorCorrecting) -> Result<(), String>{
    if match file_path.metadata() { //path is no file
        Ok(n) => !n.is_file(),
        Err(e) => return Err(e.to_string()),
    } {
        return Err("path does not lead to a file".to_string()) 
    } 
    if save_path.exists() { //path already exists
        return Err("path to save already exists".to_string()) 
    }

    let bytes = match read(file_path) { //read bytes from file
        Ok(n) => n,
        Err(e) => return Err(format!("error while reading from file; {}", e).to_string())
    };

    //implement error correcting
    let bytes = match encode_error_correcting(bytes, error_correcting_algorithm) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    //encode binary
    let dna_result = match algorithm { //contains a DNA-Sequence
        Encode::EasyEncode => easy_encode(bytes),
    };
    let dna = match &dna_result {
        Ok(_) => dna_result.unwrap(),
        Err(e) => return Err(e.to_owned()),
    };

    let file_name = match file_path.file_name() {
        Some(n) => match n.to_str() {
            Some(n) => n,
            None => return Err("file name contains invalid characters".to_string())
        },
        None => return Err("file invalid".to_string()),
    };
    let file = convert_to_fasta(&dna, &Some(&[&dna.len().to_string(), &file_name.to_string()]));
    match std::fs::write(save_path, file) {
        Err(_) => return Err("errror while writing to file".to_string()),
        _ => (),
    };

    println!("successfully converted and saved file!"); 
    Ok(())
}

///decodes a file at the given path with the given algorithm and saves it in the given path
/// 
/// returns an error if:
///     - path does not lead to a file
///     - path to save already exists
///     - given file is not a fasta file
///     - error while reading file
///     - error while writing file
fn decode_file(file_path: &std::path::Path, save_path: &std::path::Path, algorithm: Decode, error_correcting_algorithm: &ErrorCorrecting) -> Result<(), String> {
    if match file_path.metadata() { //path is not a file
        Ok(n) => !n.is_file(),
        Err(e) => return Err(e.to_string()),
    } {
        return Err("path does not lead to a file".to_string())
    } 
    if save_path.exists() { //path already exists
        return Err("path to save to already exists".to_string())
    }

    //remove header and new lines
    let sequenc = match fs::read_to_string(file_path) {
        Ok(n) => {
            let index = match n.find("\n") { //end of first line
                Some(n) => n,
                None => return Err("File fault: it does not follow the fasta format ".to_string()),
            };
            n[index+1..].replace("\n", "")
        },
        Err(_) => return Err("error while reading file. Please try again.".to_string()), 
    };

    let binary = match algorithm {
        Decode::EasyDecode => easy_decode(&sequenc),
    };
    let binary = match binary { //contains the binary version of the sequence
        Ok(n) => {
            match decode_error_correcting(n, error_correcting_algorithm) {//reverse the error correcting
                Ok(n) => n,
                Err(e) => return Err(e),
            }
        },
        Err(e) => return Err(e),
    };

    if let Err(_) = std::fs::write(save_path, binary) {
        return Err("error while writing to file".to_string())
    }

    println!("successfully converted and saved file!");
    Ok(())
}


/// gets a DNA-Sequence and converts it to the fasta format
///
/// content: DNA-Sequence
/// options: optional informationen for the header
fn convert_to_fasta<T: Display>(content: &str, options: &Option<&[T]>) -> String {
    //create header
    let mut headline = ">".to_string();
    if let Some(n) = options { //expand header if necessary
        for option in *n {
            headline += format!("{}|", option).as_str();
        }
    }

    //splits the sequence in lines, each containing 80 characters
    let elements_per_line = 80;
    let mut output = headline + "\n";
    for (index, char) in content.char_indices() {
        output.push(char);
        if (index + 1) % elements_per_line == 0 {
            output.push_str("\n");
        }
    }
    output
}

/// gets byte and implements the given error correcting algorithm for the bytes and retusn them
/// 
/// returns an error if the given error correction algorithm returns an error
fn encode_error_correcting(byte: Vec<u8>, algorithm: &ErrorCorrecting) -> Result<Vec<u8>, String> {
    let output = match algorithm {
        ErrorCorrecting::None => return Ok(byte),
        ErrorCorrecting::Hamming => hamming_encode(&byte),
    };
    match output {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("error while implementing error correcting code: {}", e).to_string()),
    }
}

fn decode_error_correcting(byte: Vec<u8>, algorithm: &ErrorCorrecting) -> Result<Vec<u8>, String> {
    let output = match algorithm {
        ErrorCorrecting::None => return Ok(byte),
        ErrorCorrecting::Hamming => hamming_decode(&byte),
    };
    match output {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("error while reversing error correcting code: {}", e).to_string()),
    }
}