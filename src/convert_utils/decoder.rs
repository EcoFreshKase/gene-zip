/*
Enthält die Funktionalität Genentische Sequenzen in Binären Code zu übersetzten. 
*/

struct StringByte{
    //ein String, der ein Byte darstellen soll 
    byte: String,
}
impl StringByte {
    #[allow(dead_code)]
    fn from_string(content: String) -> Result<StringByte, String> {
        //Gibt ein String_byte Objekt zurück, außer der gegebene String
        //ist länger als 8 oder enthält etwas anderes als 1 oder 0
        if content.len() > 8 {
            return Err("string contains to many characters".to_string())
        }
        for char in content.chars() {
            if char != '1' && char != '0' {
                return Err("string contains invalid characters".to_string())
            }
        }
        return Ok(StringByte { byte: content })
    }

    fn new() -> StringByte {
        return StringByte { byte: "".to_string() }
    }

    fn push(&mut self, ch: char) {
        self.byte.push(ch);
    }

    fn to_u8(self) -> u8 {
        //returns itself as a Byte stored in a Vec<u8>
        //consumes itself
        let mut val = 0; //contains the value of self.byte as a decimal number
        for (i, char) in self.byte.chars().into_iter().rev().enumerate() {
            if char == '0' { //überspringe 0 Bits
                continue;
            }
            let base: u8 = 2;
            val += base.pow(i as u32);
        }
        return val
    }
}

///Übersetzt eine Gen-Sequenz in Binär:
///    A -> 00
///    T -> 01
///    C -> 10
///    G -> 11
///Gegenstück zu der easy_encode Methode
///
///Gibt ein Result-Type zurück
/// string stellt eine Gensequenz dar
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
            _ => {
                return Err("Fehlerhaftes Symbol gefunden".to_string())
            }
        }
        Ok(())
    }

    let mut output: Vec<u8> = Vec::new();

    let mut iterator = string.chars();
    let mut condition = true; //false wenn das Ende der Gen-Sequenz erreicht wird
    while condition {
        let chars = [iterator.next(), iterator.next(), iterator.next(), iterator.next()]; //speichert 4 Nukleotide
        let mut byte = StringByte::new(); //Speichert den aktuellen Byte

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

    return Ok(output)
}