/*
Enthält eine builder Funktion für den Button, mit dem die Konvertierung gestartet wird
*/

use std::fmt::Display;
use std::path::Path;
use std::fs;
use druid::{Widget, EventCtx, Env};
use druid::widget::{Button};
use super::AppState::AppState;
use super::{AlgorithmType, Decode, Encode};
use super::{open_error, loading_window::open_loading};
use crate::convert_utils::{easy_encode, easy_decode};
use crate::ERROR;

pub fn start_button_builder() -> impl Widget<AppState> {
    let button = Button::new("Convert")
        .on_click(|ctx: &mut EventCtx, data: &mut AppState, env: &Env| {
            if let AlgorithmType::None = data.algorithm_type{ //Fehlermeldung wenn kein Algorithmus ausgewählt ist
                open_error(ctx, data, env, "Wähle einen Algorithmus aus!");
                return;
            }
            if let None = data.decode_algorithm { //Fehlermeldung wenn kein Algorithmus ausgewählt ist
                if let None = data.encode_algorithm {
                    open_error(ctx, data, env, "Wähle einen Algorithmus aus!");
                    return;
                }
            }

            //Konvertieren der Datei
            data.calculating = true;
            open_loading(ctx, data, env);
            let mut error_msg = None; //Speichert eventuelle Fehlernachrichten
            match data.algorithm_type {
                AlgorithmType::Encode => match encode_file(
                Path::new(&data.file_path),
                Path::new(&data.save_path), 
                data.encode_algorithm.clone().unwrap()) {
                    Ok(_) => (),
                    Err(e) => error_msg = Some(e),
                },
                AlgorithmType::Decode => match decode_file(
                Path::new(&data.file_path),
                Path::new(&data.save_path), 
                data.decode_algorithm.clone().unwrap()) {
                    Ok(_) => (),
                    Err(e) => error_msg = Some(e),
                },
                AlgorithmType::None => error_msg = Some("Wähle einen Algorithmus aus!".to_string()),
            }
            if let Some(_) = error_msg {
                ctx.submit_command(ERROR);
                open_error(ctx, data, env, error_msg.unwrap());
            }
            data.calculating = false;
        });
    button
}

fn encode_file(file_path: &std::path::Path, save_path: &std::path::Path, algorithm: Encode) -> Result<(), String>{
    /*
    Erhält alle notwendigen Informationen, um eine gegeben Datei
    zu einer DNA-Sequenz zu übersetzen und diese zu speichern.
    */
    //überprüfen ob die gegebenen Pfade zulässig sind
    if match file_path.metadata() { //Pfad ist keine Datei
        Ok(n) => !n.is_file(),
        Err(e) => return Err(e.to_string()),
    } {
        return Err("Gegebene Datei ist keine Datei".to_string())
    } 
    if save_path.exists() { //Pfad existiert schon
        return Err("Pfad, in welchem gespeichert werden soll, existiert schon".to_string())
    }

    let file_path_str = match file_path.as_os_str().to_str() { //file_path zu &str konvertieren
        Some(n) => n,
        None => return Err("file_path Fehlerhaft, versuche es erneut".to_string()),
    };

    let dna_result = match algorithm { //Enthält die DNA-Sequenz
        Encode::EasyEncode => easy_encode(file_path_str),
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
        Err(_) => return Err("Fehler beim Schreiben der Datei".to_string()),
        _ => (),
    };

    println!("Erfolgreich Datei konvertiert und gespeichert!");
    Ok(())
}

fn decode_file(file_path: &std::path::Path, save_path: &std::path::Path, algorithm: Decode) -> Result<(), String> {
    /*
    Erhält alle notwendigen Informationen, um eine gegebene DNA-Sequenz zu einer
    binären Datei zu konvertieren und diese zu speichern.
    */
    //überprüfen ob die gegebenen Pfade zulässig sind
    if match file_path.metadata() { //Pfad ist keine Datei
        Ok(n) => !n.is_file(),
        Err(e) => return Err(e.to_string()),
    } {
        return Err("Gegebene Datei ist keine Datei".to_string())
    } 
    if save_path.exists() { //Pfad existiert schon
        return Err("Pfad, in welchem gespeichert werden soll, existiert schon".to_string())
    }

    //Header und Zeilenumbrüche entfernen
    let sequenc = match fs::read_to_string(file_path) {
        Ok(n) => {
            let index = match n.find("\n") { //Ende der ersten Zeile
                Some(n) => n,
                None => return Err("Datei Fehlerhaft: Sie entspricht nicth dem FASTA-Format".to_string()),
            };
            n[index+1..].replace("\n", "")
        },
        Err(_) => return Err("Es ist ein Fehler beim Lesen der Datei aufgetreten.\nVersuche es erneut.".to_string()),
    };

    let binary = match algorithm {
        Decode::EasyDecode => easy_decode(&sequenc),
    };
    let binary = match binary { //Enthält die Binäre Sequenz der Datei
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    if let Err(_) = std::fs::write(save_path, binary) {
        return Err("Fehler beim Schreiben der Datei".to_string())
    }

    println!("Erfolgreich Datei konvertiert und gespeichert!");
    Ok(())
}


///Erhlält eine DNA-Sequenz als String und konvertiert sie zu einem String,
///welcher dem FASTA-Format entspricht
/// content: DNA Sequenz
/// options: optionale Informationen, die in die Headliner getragen werden, getrennt von einem |
fn convert_to_fasta<T: Display>(content: &str, options: &Option<&[T]>) -> String {
    //Headline erstellen
    let mut headline = ">".to_string();
    if let Some(n) = options { //Headline erweitern wenn nötig
        for option in *n {
            headline += format!("{}|", option).as_str();
        }
    }


    //Teilt die Gensequenz in Zeilen auf, die jeweils bis zu 80 Nukleotide enthalten
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
