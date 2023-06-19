/*
contains functions to translate binary to DNA sequences
*/

use std::io::{BufReader, Read, ErrorKind};
use std::fs::File;
use std::thread;
use std::time;

///translates binary to DNA with the following pattern:
/// 00 -> A
/// 01 -> T
/// 10 -> C
/// 11 -> G
///counterpart to the easy_decode function
///
///takes a path to a file as an &str and returns a Result.
///
///Error might occour if an buf_read couldn't read from the file or if the binary sequence couldn't be converted to DNA.
///Every file can be converted to DNA but an error is returned when the buf_read returns a string that not only contains 1 and 0.
pub fn easy_encode(path: &str) -> Result<String, String> {
    let mut reader = match get_file_reader(path){
        Ok(bufreader) => bufreader,
        Err(e) => return Err(e),
    };
    let mut buffer = [0]; //stores every single bit
    let mut output = String::new();

    //reads every byte and converts it into DNA
    while match buf_read(&mut reader, &mut buffer) {
        Ok(n) => n,
        Err(e) => return Err(String::from(format!("Error while reading from the file: {}", e.kind())))
    } != 0 {
        let byte = format!("{:08b}", buffer[0]);
        let mut chars = byte.chars(); //stores all digits as an iterator

        //iterate through all bytes from most significant to least siginificant
        let mut gens: String = String::new();
        for _ in 1..5 {
            let bit1 = match chars.next() {
                Some(n) => n,
                None => break,
            };
            let bit2 = match chars.next () {
                Some(n) => n,
                None => '0', //shouldn't happen but you never know!
            };
            let bits = bit1.to_string() + &bit2.to_string();
            let gen = match bits.as_str() { //matches 2 bits to a nucleotide
                "00" => 'A',
                "01" => 'T',
                "10" => 'C',
                "11" => 'G',
                _ => return Err(String::from(format!("Critical failure while converting from binary to a genome-sequence. Byte can not be matched to a Gen. Byte: {}, Failure bits: {}", byte, bits)))
            };
            gens.push(gen); //save the nucleotide
        }
        output += &gens;
    };
    return Ok(output)
}

///gets a path to a file as a string and returns a BufReader of the file
///
/// returns an error if the file couldn't be opened
pub fn get_file_reader(path: &str) -> Result<BufReader<File>, String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(String::from(format!("Error while opening file: {}", e.kind())))
    };

    let reader = BufReader::new(file);
    return Ok(reader)
}

///like BufReader.read() but guarentees that every returns error is fatal and not ErrorKind::Interrupted
/// -> lets the thread sleep for 10ms and then tries again until it works
pub fn buf_read<T: std::io::Read>(reader: &mut BufReader<T>, mut buffer: &mut [u8]) -> std::io::Result<usize> {

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