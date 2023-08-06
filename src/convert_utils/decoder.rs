/*
contains functions to convert DNA-Sequences to binary.
*/

//a string representing a Byte
struct StringByte{
    byte: String,
}
impl StringByte {
    
    /// creates a StringByte from a string
    /// error if the string has more then 8 characters or contains characters that are not 1 or 0
    #[allow(dead_code)]
    fn from_string(content: String) -> Result<StringByte, String> {
        if content.len() > 8 {
            return Err("string contains to many characters".to_string())
        }
        for char in content.chars() {
            if char != '1' && char != '0' {
                return Err("string contains invalid characters".to_string())
            }
        }
        Ok(StringByte { byte: content })
    }

    fn new() -> StringByte {
        StringByte { byte: "".to_string() }
    }

    fn push(&mut self, ch: char) {
        self.byte.push(ch);
    }

    //returns the represented byte as a u8
    fn to_u8(&self) -> u8 {
        let mut val = 0; //contains the value of self.byte as a decimal number
        for (i, char) in self.byte.chars().rev().enumerate() {
            if char == '0' { //skip 0 bits
                continue;
            }
            let base: u8 = 2;
            val += base.pow(i as u32);
        }
        val
    }
}

/// converts a DNA-Sequence to binary:
///    A -> 00
///    T -> 01
///    C -> 10
///    G -> 11
/// 
/// returns an error if the given string contains other characters then 0 and 1
pub fn easy_decode(string: &str) -> Result<Vec<u8>, String> {
    fn match_char(stringbyte: &mut StringByte, nucleotide: char) -> Result<(), String>{
        match nucleotide {
            'A' => {
                stringbyte.push('0');
                stringbyte.push('0');
            }
            'T' => {
                stringbyte.push('0');
                stringbyte.push('1');
            }
            'C' => {
                stringbyte.push('1');
                stringbyte.push('0');
            }
            'G' => {
                stringbyte.push('1');
                stringbyte.push('1');
            }
            _ => return Err(format!("invalid symbol found while decoding: {}", nucleotide))
        }
        Ok(())
    }

    let mut output: Vec<u8> = Vec::new();

    let mut iterator = string.chars();
    let mut condition = true; //check if the end of the sequence is reached
    while condition {
        let chars = [iterator.next(), iterator.next(), iterator.next(), iterator.next()]; //stores 4 nucleotides
        let mut byte = StringByte::new(); //contains the current byte

        for char in chars {
            match char {
                Some(n) => match match_char(&mut byte, n) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                },
                None => {
                    condition = false;
                    break;
                }
            }
        }
        output.push(byte.to_u8())
    }

    Ok(output)
}