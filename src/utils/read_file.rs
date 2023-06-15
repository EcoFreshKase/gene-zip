/*
Enthält Funktionalität, um den Inhalt einer Datei zu lesen.
*/
use std::fs::File;
use std::io::{BufReader, Read, ErrorKind};
use std::thread;
use std::time;

pub fn get_file_reader(path: &str) -> Result<BufReader<File>, String> {
    /*
    Erhält einen Pfad zu einer Datei und gibt ein BufReader-Objekt zurück, mit dem der Inhalt der Datei gelesen werden kann.
    */
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(String::from(format!("Error while opening file: {}", e.kind())))
    };

    let reader = BufReader::new(file);
    return Ok(reader)
}

pub fn buf_read<T: std::io::Read>(reader: &mut BufReader<T>, mut buffer: &mut [u8]) -> std::io::Result<usize> {
    /*
    Benutzt die read Methode auf das erhaltene BufReader-Objekt. Es wird garantiert, dass jeder Fehler, der
    zurückgegeben wird fatal ist und nicht durch ErrorKind::Interrupted verursacht wurde.

    Wenn reader.read den Fehler: ErrorKind::Interrupted zurück gibt wird die Funktion rekursiv alle 10ms aufgerufen
    bis die Datei gelesen werden konnte.
    */

    match reader.read(&mut buffer) {
        Ok(n) => return Ok(n),
        Err(e) => match e.kind() {
            ErrorKind::Interrupted => {
                println!("Interrupted while reading file. Trying again...");
                thread::sleep(time::Duration::from_millis(10));
                buf_read(reader, &mut buffer)
            },
            _ => return Err(e)
        }
    }
}