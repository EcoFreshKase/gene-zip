/*
Enthält eine builder Funktion für den Button, mit dem die Konvertierung gestartet wird

contains a builder function for the button that starts the conversion
*/

use std::fmt::Display;
use std::path::Path;
use std::fs::{self, read};
use std::thread;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;
use std::str;
use druid::{Widget, EventCtx, Env, Event, WidgetExt, TimerToken, Target};
use druid::widget::{Button, Controller};
use super::AppState::AppState;
use super::error_correcting::ErrorCorrecting;
use super::{AlgorithmType, Decode, Encode};
use super::{open_error, loading_window::open_loading};
use crate::{START_CONVERSION, GLOBAL_UPDATE};
use crate::convert_utils::{easy_encode, easy_decode};
use crate::convert_utils::error_correcting::hamming_code::{hamming_decode, hamming_encode};

#[derive(Debug)]
enum ConversionStatus { //Used to communicate between threads
    Res(f64, String), // Result during the conversion. Contains an value added to AppState::calculating an a Message to display.
    End(Result<(), String>), // End of the Conversion.
}

struct ConversionHandler { //handles the multithreaded conversion of files
    timer_id: TimerToken,
    receiver: Option<mpsc::Receiver<ConversionStatus>>
}

impl ConversionHandler {
    fn new() -> ConversionHandler {
        ConversionHandler {
            timer_id: TimerToken::INVALID,
            receiver: None,
        }
    }
}

impl<W: Widget<AppState>> Controller<AppState, W> for ConversionHandler {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(START_CONVERSION) => { //start of conversion
                match data.algorithm_type { //error message if no algorithm is selected
                    AlgorithmType::Decode => {
                        if let None = data.decode_algorithm {
                            open_error(ctx, data, env, "Choose an algorithm!");
                            return;
                        }
                    }
                    AlgorithmType::Encode => {
                        if let None = data.encode_algorithm {
                            open_error(ctx, data, env, "Choose an algorithm!");
                            return;
                        }
                    }
                    AlgorithmType::None => {
                        open_error(ctx, data, env, "Choose an algorithm!");
                        return;
                    }
                }

                //conversion of the file
                data.calculating = 0.0;
                data.error_msg.clear(); //clear any prior error messages
                data.calculating_msg = "starting conversion ...".to_string();
                open_loading(ctx, data, env);
                let (tx, rx) = mpsc::channel::<ConversionStatus>();
                self.receiver = Some(rx);

                let data_clone = data.clone();
                match data.algorithm_type {
                    AlgorithmType::Encode => {
                        thread::spawn(move || {
                            let header = match &data_clone.get_header() { // get FASTA header
                                Ok(n) => n.to_owned(),

                                // End the execution if no header could be created,
                                Err(e) => return tx.send(ConversionStatus::End(Err(e.to_owned()))) 
                            };

                            return match encode_file(
                                Path::new(&data_clone.file_path),
                                Path::new(&data_clone.save_path), 
                                data_clone.encode_algorithm.unwrap(), //safe to call unwrap because it was checked earlier
                                &data_clone.error_correcting,
                                tx.clone(),
                                header
                            ) {
                                Ok(_) => tx.send(ConversionStatus::End(Ok(()))),
                                Err(e) => tx.send(ConversionStatus::End(Err(e))),
                            };
                            });
                    },
                    AlgorithmType::Decode => {
                        thread::spawn(move || {
                            match decode_file(
                                Path::new(&data_clone.file_path),
                                Path::new(&data_clone.save_path), 
                                data_clone.decode_algorithm.unwrap(), //safe to call unwrap because it was checked earlier
                                &data_clone.error_correcting,
                                tx.clone()) {
                                Ok(_) => tx.send(ConversionStatus::End(Ok(()))),
                                Err(e) => tx.send(ConversionStatus::End(Err(e))),
                            }
                        });
                    },

                    AlgorithmType::None => open_error(ctx, data, env, "Choose an algorithm!".to_string()),
                }

                self.timer_id = ctx.request_timer(Duration::from_millis(100)); //set timer to check for status
            },

            Event::Timer(id) if *id == self.timer_id => {
                let rx = match &self.receiver { //channel with conversion thread
                    Some(n) => n,
                    None => { //
                        open_error(ctx, data, env, "Error while converting file: Receiver was not created when receiving message");
                        return child.event(ctx, event, data, env)
                    }
                };

                match get_last_update(&rx) {
                    Ok(n) => match n { //check which status the conversion currently has
                        ConversionStatus::Res(progress, msg) => {
                            data.calculating += progress;
                            data.calculating_msg = msg.clone();
                            ctx.submit_command(GLOBAL_UPDATE);

                            // The conversion can't be done if a ConversionStatus::Res is received
                            if data.calculating >= 1.0 {
                                data.calculating = 0.9;
                            }
                        },

                        ConversionStatus::End(result) => {
                            match result {
                                Ok(_n) => {
                                    data.calculating = 1.0;
                                    data.calculating_msg = "successfully converted and saved file!".to_string();
                                    ctx.submit_command(GLOBAL_UPDATE);
                                    println!("successfully converted and saved file!");
                                },
                                Err(e) => {
                                    println!("received an error: {}", e);
                                    data.calculating = 0.0;
                                    data.error_msg = e;
                                    ctx.submit_command(GLOBAL_UPDATE);
                                }
                            }
                            return
                        }
                    },
                    Err(e) => match e {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => {
                            data.calculating = 0.0;
                            data.error_msg = "Error while converting file: multithreading channel unexpectedly closed".to_string();
                        },
                    },
                }

                if data.calculating != 1.0 { //new timer if conversion is not completed
                    self.timer_id = ctx.request_timer(Duration::from_millis(100));
                }
            },

            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}

pub fn start_button_builder() -> impl Widget<AppState> {
    let button = Button::new("Convert")
        .on_click(|ctx: &mut EventCtx, _data: &mut AppState, _env: &Env| {
            ctx.submit_command(START_CONVERSION.to(Target::Global));
        }).controller(ConversionHandler::new());
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
fn encode_file<T: Display>(file_path: &std::path::Path, save_path: &std::path::Path, algorithm: Encode, error_correcting_algorithm: &ErrorCorrecting, tx: std::sync::mpsc::Sender<ConversionStatus>, header: T) -> Result<(), String> {
    if let Err(e) = check_paths(file_path, save_path) {
        return Err(e)
    }

    let bytes = match read(file_path) { //read bytes from file
        Ok(n) => n,
        Err(e) => return Err(format!("error while reading from file; {}", e).to_string())
    };

    //implement error correcting
    println!("implementing error_correcting ...");
    match tx.send(ConversionStatus::Res(0.0, "implementing error_correcting ...".to_string())) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string())
    }
    let bytes = match encode_error_correcting(bytes, error_correcting_algorithm) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    //encode binary
    let file_size = match file_path.metadata(){
        Ok(n) => n.len(),
        Err(e) => return Err(format!("error when accessing metadata: {}", e)),
    };
    let chunk_size = get_chunk_size(&(file_size as usize));
    let mut dna = String::new();
    println!("start of conversion");
    match tx.send(ConversionStatus::Res(chunk_size as f64/file_size as f64, "converting binary to DNA ...".to_string())) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string())
    };
    for byte_chunks in bytes.chunks(chunk_size) { //encoding bytes in chunk_size chunks
        let result = match algorithm {
            Encode::EasyEncode => easy_encode(byte_chunks.to_vec()),
        };
        match result {
            Ok(n) => {
                match tx.send(ConversionStatus::Res(chunk_size as f64/file_size as f64, "converting binary to DNA ...".to_string())) {
                    Ok(_) => (),
                    Err(e) => return Err(e.to_string())
                };
                dna.push_str(&n);
            },
            Err(e) => return Err(e)
        }
    }
    println!("end conversion");

    match tx.send(ConversionStatus::Res(0.0, "converting DNA sequence to FASTA ...".to_string())) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string())
    }
    let file = convert_to_fasta(&dna, &header);
    
    match tx.send(ConversionStatus::Res(0.0, "saving to a file ...".to_string())) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string())
    }
    match std::fs::write(save_path, file) {
        Err(_) => return Err("errror while writing to file".to_string()),
        _ => (),
    };

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
fn decode_file(file_path: &std::path::Path, save_path: &std::path::Path, algorithm: Decode, error_correcting_algorithm: &ErrorCorrecting, tx: std::sync::mpsc::Sender<ConversionStatus>) -> Result<(), String> {
    if let Err(e) = check_paths(file_path, save_path) {
        return Err(e)
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

    let chunk_size = { // Has to be a multiple of 4
        let size = get_chunk_size(&sequenc.len());
        println!("{} {} {}", sequenc.len(), size, size%4);
        let size = size - size%4 + 4; // Remove the remainder to make the size a multiple of 4. Add 4 to prevent the size from becoming 0.
        size
    };
    let mut binary: Vec<u8> = Vec::new();
    let seq_iterator = sequenc.as_bytes() //iterator over chunks of the sequenc
        .chunks(chunk_size)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    
    if let Err(e) = tx.send(ConversionStatus::Res(chunk_size as f64/sequenc.len() as f64, "converting DNA to binary ...".to_string())) {
        return Err(e.to_string())
    };
    for sequenc_chunk in seq_iterator {
        let res = match algorithm {
            Decode::EasyDecode => easy_decode(&sequenc_chunk),
        };
        match res { //contains the binary version of the sequence
            Ok(mut n) => binary.append(&mut n),
            Err(e) => return Err(e),
        }
        match tx.send(ConversionStatus::Res(chunk_size as f64/sequenc.len() as f64, "converting DNA to binary ...".to_string())) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string())
        };
    }

    match tx.send(ConversionStatus::Res(0.0 , "reversing error correcting ...".to_string())) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string())
    };
    let binary = match decode_error_correcting(binary, error_correcting_algorithm) { //reverse the error correcting
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    match tx.send(ConversionStatus::Res(0.0 , "saving to a file ...".to_string())) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string())
    };
    if let Err(_) = std::fs::write(save_path, binary) {
        return Err("error while writing to file".to_string())
    }
    Ok(())
}

/// Checks if the given Paths is valid
fn check_paths (file_path: &std::path::Path, save_path: &std::path::Path) -> Result<(), String> {
    if match file_path.metadata() { //path is no file
        Ok(n) => !n.is_file(),
        Err(e) => return Err(e.to_string()),
    } {
        return Err("path does not lead to a file".to_string()) 
    } 
    if save_path.exists() { //path already exists
        return Err("save path already exists".to_string()) 
    }
    Ok(())
}

/// gets a DNA-Sequence and converts it to the fasta format
///
/// content: DNA-Sequence
/// header: The Fasta Header
fn convert_to_fasta<T: Display>(content: &str, header: &T) -> String {

    //splits the sequence in lines, each containing 80 characters
    let elements_per_line = 80;
    let mut output = header.to_string() + "\n";
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

/// Returns a size for the chunks in which the file to convert gets split
fn get_chunk_size(size: &usize) -> usize {
    let mut out = size/20; //chunks size is 5% of total size
        
    // When the size is smaller than 100 units set the chunk_size to size.
    if out <= 0 {
        out = *size;
    }
    out
}

/// Receives all available values of this receiver but only returns the last received value.
/// Returns an error if there is no available value to receive.
fn get_last_update(rx: &mpsc::Receiver<ConversionStatus>) -> Result<ConversionStatus, mpsc::TryRecvError> {
    let mut last_value: Option<ConversionStatus> = None;
    loop {
        match rx.try_recv() {
            Ok(n) => last_value = Some(n),
            Err(e) => {
                if let None = last_value {
                    return Err(e)
                }
                break
            }
        }
    }

    Ok(last_value.unwrap())
}