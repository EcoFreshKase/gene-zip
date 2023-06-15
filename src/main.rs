//#![windows_subsystem = "windows"]

use std::env;
use std::path::Path;
use druid::{WindowDesc, AppLauncher, Selector, Command};

mod convert_utils;
mod utils;
mod gui_builder;

use gui_builder::decode_encode;

const WINDOW_SIZE: (f64, f64) = (599.0 *1.3, 337.0 *1.3); //Größe des Fensters (Breite, Höhe)
const ERROR: Selector = Selector::new("ERROR WHILE CONVERTING");

fn main() {

    //let args: Vec<String> = env::args().collect();
    let args: Vec<String> = vec![String::from("C:\\Users\\alexa\\OneDrive\\Dokumente\\Schule\\5. PK\\g-zip\\dummy_file.txt")];
    //let args: Vec<String> = vec![String::from("D:\\Bilder\\Insta\\convert.7z")];
    let path = args.get(0).unwrap();

    //default Werte für den Startzustand der Anwendung
    let default_save_extension= ".fasta";
    let default_save_path = { //Standard Pfad für die Speicherung der konvertierten Datei
        let extension_index = { //Start des Datei Typs
            let mut tmp = None; //speichert den Rückgabe Wert
            for (index, char) in path.chars().rev().enumerate() { //iterate through the path from behind
                if char == '.' {
                    tmp = Some(path.len() - index - 1);
                    break;
                } else if char == '\\' {
                    tmp = Some(path.len());
                    break;
                }
            }
            tmp.expect(&format!("Path is not valid: {}", path))
        };
        path[..extension_index].to_owned() + default_save_extension
    };
    let default_algorithm_type = { //Standardmäßig ausgewählter Algorithmus-Typ
        let path = Path::new(path);
        let mut output = decode_encode::AlgorithmType::Encode;

        if let Some(extension) = path.extension() {
            if extension == "fasta" {
                output = decode_encode::AlgorithmType::Decode;
            }
        }
        output
    };

    //GUI Erstellung
    let main_window = WindowDesc::new(gui_builder::build_ui())
        .title("g-zip")
        .resizable(false)
        .window_size(WINDOW_SIZE);
    let initial_state = gui_builder::AppState::AppState {
        file_path: path.to_owned(),
        save_path: default_save_path,
        error_correcting: gui_builder::error_correcting::ErrorCorrecting::None,
        algorithm_type: default_algorithm_type,
        decode_algorithm: None,
        encode_algorithm: None,
        calculating: false,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}