/*
Enthält die Funktionalität Binären Code in Genentische Sequenzen zu übersetzten. 
*/

use std::io::{BufReader, Read, ErrorKind};
use std::fs::File;
use std::thread;
use std::time;

pub fn easy_encode(path: &str) -> Result<String, String> {
    /*
    Übersetzt Binär in Genentische Sequenzen:
        00 -> A
        01 -> T
        10 -> C
        11 -> G
    Gegenstück zu der easy_decode Methode

    Erhält einen Pfad zu einer Datei und gibt einen String zurück der die Datei in Gene überesetzt.
    */
    let mut reader = match get_file_reader(path){
        Ok(bufreader) => bufreader,
        Err(e) => return Err(e),
    };
    let mut buffer = [0]; //speichert jeden einzelnen Byte zwischen
    let mut output = String::new();

    //liest jeden Byte einzeln und wandelt ihn in eine Basen-Sequenz um
    while match buf_read(&mut reader, &mut buffer) {
        Ok(n) => n,
        Err(e) => return Err(String::from(format!("Error while reading from the file: {}", e.kind())))
    } != 0 {
        let byte = format!("{:08b}", buffer[0]);
        let mut chars = byte.chars(); //Speichert alle Ziffern des Bytes in einem Iterator

        /*durch alle Bits des Bytes iterieren
        Bits werden von "links" nach "rechts" iteriert für den Fall 
        das der Byte eine ungerade Anzahl an Ziffern hat wird "links"
        eine weitere 0 hinzugefügt für die Übersetzung in eine Gen-Sequenz
        */
        let mut gens: String = String::new(); //Speichert alle Basen für ein Byte
        for _ in 1..5 {
            let bit1 = match chars.next() {
                Some(n) => n,
                None => break,
            };
            let bit2 = match chars.next () {
                Some(n) => n,
                None => '0', //sollte nicht auftreten aber für den Fall der Fälle
            };
            let bits = bit1.to_string() + &bit2.to_string();
            let gen = match bits.as_str() { //speichert 2 bits konvertiert zu einer Nukleinsäure
                "00" => 'A',
                "01" => 'T',
                "10" => 'C',
                "11" => 'G',
                _ => return Err(String::from(format!("Critical failure while converting from binary to a genome-sequence. Byte can not be matched to a Gen. Byte: {}, Failure bits: {}", byte, bits)))
            };
            gens.push(gen); //speichert die Nukleinsäure zu der Basen-Sequenz
        }
        output += &gens;
    };
    return Ok(output)
}

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