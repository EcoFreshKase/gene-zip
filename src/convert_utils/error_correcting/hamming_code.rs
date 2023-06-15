/*
enthält Funktionen, um Dateien mit fehlerkorrigierenden Codes zu versehen

Mögliche Quelle zum inspirieren:
https://github.com/JuxhinDB/hamming-code/blob/main/src/hamming.rs
*/

///Stellt einen (7,4)-Hamming Code dar
/// data_bits enthält alle Daten Bits
/// parity_bits enthält alle parität Bits
struct HammingCode {
    data_bits: [u8; 4],
    parity_bits: [u8; 3],
}

impl HammingCode {
    fn new(bits: [u8; 4]) -> HammingCode {
        for bit in bits {
            if bit > 2 {
                panic!("{:?} enhält {} ein ungültiger Bit während der Erstellung eines Hamming Codes", bits, bit)
            }
        }

        HammingCode { 
            data_bits: bits, 
            parity_bits: HammingCode::get_parity_bits(&bits)
        }
    }

    ///returns 2 HammingCode-Types, because 1 HammingCode-Type only stores 4 bits
    /// 
    /// u8: #1 #2 #3 #4 #5 #6 #7 #8
    /// HammingCode Databits: [#1, #2, #3, #4], [#5, #6, #7, #8]
    fn from_u8(u8: &u8) -> [HammingCode; 2] {
        //convert u8 into bits
        let mut bits = [0,0,0,0,0,0,0,0];
        let mut output: [HammingCode; 2];
        for i in 0..8 {
            bits[i] = (u8 >> i) & 1; //Schwarze Magie von: https://stackoverflow.com/questions/74162324/how-do-i-convert-an-integer-to-binary-in-rust-such-that-i-can-iterate-over-the
        }

        [HammingCode::new([bits[0], bits[1], bits[2], bits[3]]), HammingCode::new([bits[4], bits[5], bits[6], bits[7]])]
    }

    ///
    fn from_hamming_u8(u8: &u8) -> HammingCode {
        let get_bit = |n| (u8 >> n) & 1u8;
        HammingCode { 
            data_bits: [get_bit(2), get_bit(5), get_bit(6), get_bit(7)],
            parity_bits: [get_bit(0), get_bit(1), get_bit(3)],
        }
    }

    /// data_bits and parity_bits conversion: d4 d3 d2 p3 d1 p2 p1
    fn to_u8(&self) -> u8 {
        let bits: [u8; 7] = [self.parity_bits[0], self.parity_bits[1], self.data_bits[0], self.parity_bits[2], self.data_bits[1], self.data_bits[2], self.data_bits[3]];
        return bits.iter().enumerate().map(|(index, bit)| bit * 2u8.pow(index as u32)).sum()
    }

    ///Berechnet die parität Bits für die gegeben Bits
    fn get_parity_bits(data_bits: &[u8; 4]) -> [u8; 3] {
        [
            (data_bits[0] + data_bits[1] + data_bits[3]) % 2, // ^ -> Bitwise XOR
            (data_bits[0] + data_bits[2] + data_bits[3]) % 2,
            (data_bits[1] + data_bits[2] + data_bits[3]) % 2,
        ]
    }

    fn get_data(&self) -> &[u8; 4] {
        &self.data_bits
    }

    ///Überprüft ob self.data_bits mit self.parity_bits Fehler aufweißt
    /// Weißt Fehler auf -> true
    /// Weißt keine Fehler auf -> false
    fn check(&self) -> bool {
        let parity_bits = HammingCode::get_parity_bits(&self.data_bits);
        if self.parity_bits[0] != parity_bits[0] { //Ersten Paritäts Bit überprüfen
            return true
        }
        if self.parity_bits[1] != parity_bits[1] { //Zweiten Paritäts Bit überprüfen
            return true
        }
        if self.parity_bits[2] != parity_bits[2] { //Dritten Paritäts Bit überprüfen
            return true
        }
        false
    }

    /// Korrigiert in eigenen Daten Bits, wenn es welche gibt.
    /// Kann nicht sicher stellen, dass das Ergebniss korrekt ist.
    ///     -> Ergebnis nur korrekt wenn es maximal einen Bit Flip gab
    ///     -> alle anderen Fälle führen zu inkorrekten Ergebnissen
    fn correct(&mut self) -> u8{
        fn flip_bit(bit: &mut u8) {
            if *bit == 1 {
                *bit = 0;
            } else {
                *bit = 1;
            }
        }

        //Gespeicherte Parität Bits mit neuen vergleichen
        let new_parity_bits = HammingCode::get_parity_bits(&self.data_bits);
        if new_parity_bits[0] != self.parity_bits[0] {
            if new_parity_bits[1] != self.parity_bits[1] {
                if new_parity_bits[2] != self.parity_bits[2] {
                    flip_bit(&mut self.data_bits[3]);
                    return 1
                }
                flip_bit(&mut self.data_bits[0]);
                return 1;
            }
            if new_parity_bits[2] != self.parity_bits[2] {
                flip_bit(&mut self.data_bits[1]);
                return 1;
            }
            flip_bit(&mut self.parity_bits[0]);
            return 1;
        }
        if new_parity_bits[1] != self.parity_bits[1] {
            if new_parity_bits[2] != self.parity_bits[2] {
                flip_bit(&mut self.data_bits[2]);
                return 1;
            }
            flip_bit(&mut self.parity_bits[1]);
            return 1;
        }
        if new_parity_bits[2] != new_parity_bits[2] {
            flip_bit(&mut self.parity_bits[2]);
            return 1;
        }
        println!("Es gab keine Fehler");
        0 // Es gab keine Fehler
    }
}

#[allow(dead_code, unused_variables)]
pub fn hamming_encode(bytes: &Vec<u8>) -> Vec<u8> {
    /*
    verseht eine Datei mit dem Hamming Code

    erhält Bytes als Vec<u8>, implementiert den (7,4) Hamming Code und gibt die neuen Bytes als Vec<u8> zurück
    */
    let mut output: Vec<u8> = Vec::new();

    for byte in bytes { //byte konvertieren
        let hammings = HammingCode::from_u8(byte);
        output.push(hammings[1].to_u8());
        output.push(hammings[0].to_u8());
    }
    output
}

#[allow(dead_code, unused_variables)]
pub fn hamming_decode(bytes: &Vec<u8>) -> (Vec<u8>, usize) {
    /*
    Korrigiert mögliche Fehler in einer Datei, die mit dem Hamming Code versehen wurde, und
    gibt die korrigierte Bytes zurück.

    Erhält Bytes als Vec<u8>, die den (7,4) Hamming Code implementiert haben.
    */
    let mut output: Vec<u8> = Vec::new();
    let mut corrections: usize = 0;

    let mut bytes_iter = bytes.iter();
    loop { //Bytes konvertieren
        let byte1 = match bytes_iter.next() {
            Some(n) => n,
            None => break,
        };
        let byte2 = match bytes_iter.next() {
            Some(n) => n,
            None => &0,
        };
        let mut hamming1 = HammingCode::from_hamming_u8(&byte1);
        let mut hamming2 = HammingCode::from_hamming_u8(&byte2);

        let correction1 = hamming1.correct();
        let correction2 = hamming2.correct();
        println!("{} {}", correction1, correction2);

        corrections += correction1 as usize;
        corrections += correction2 as usize;

        let bits: Vec<&u8> = hamming1.get_data().iter().chain(hamming2.get_data().iter()).collect();
        let sum: u8 = bits.iter().enumerate().map(|(i, b)| *b * 2u8.pow(i as u32)).sum();
        output.push(sum);
    }
    (output, corrections)
}

#[cfg(test)]
mod test {
    use super::{HammingCode, hamming_decode, hamming_encode};

    /// Alle Kombinationen von 4 Bits als Iterator
    struct All4Bits {
        bits: [u8; 4],
    }
    impl All4Bits {
        fn new() -> All4Bits {
            All4Bits { bits: [0,0,0,0] }
        }
    }
    impl Iterator for All4Bits {
        type Item = [u8; 4];

        fn next(&mut self) -> Option<Self::Item> {
            if self.bits[0] == 0 {
                self.bits[0] = 1;
                return Some(self.bits)
            } else if self.bits[1] == 0 {
                self.bits[1] = 1;
                return Some(self.bits)
            } else if  self.bits[2] == 0 {
                self.bits[2] = 1;
                return Some(self.bits)
            } else if self.bits[3] == 0 {
                self.bits[3] = 1;
                return Some(self.bits);
            }
            None
        }
    }

    fn flip_bit(bit: &mut u8) {
        if *bit == 1 {
            *bit = 0;
        } else {
            *bit = 1;
        }
    }

    //flipt den n-ten Bit eine Bytes
    fn flip_n_bit(bit: &mut u8, n: usize) -> u8 {
        let output: u8;
        if (*bit >> n) & 1 == 1 { //Überprüfen ob Bit 1 ist
            let rev_bit = !*bit;
            output = !(rev_bit | (1 << n));
        } else {
            output = (*bit | (1 << n));
        }
        *bit = output;
        output
    }

    #[test]
    fn creation_of_parity_bits() {
        let mut bits = [1,1,0,0];

        assert_eq!(bits, HammingCode::new(bits).data_bits);
        assert_eq!([0,1,1], HammingCode::new(bits).parity_bits);

        bits = [1,0,1,1];
        assert_eq!([0,1,0], HammingCode::get_parity_bits(&bits));
    }

    #[test]
    fn error_finding() {
        let mut hamming_code = HammingCode{
            data_bits: [1,1,1,1],
            parity_bits: [1,0,0],
        };

        assert!(hamming_code.check(), "failed at detecting an error");
        
        hamming_code.parity_bits = [1,1,1];
        assert!(!hamming_code.check(), "falsely detected an error");

        for bits in All4Bits::new() {
            let mut hamming_code = HammingCode::new(bits);
            assert!(!hamming_code.check(), "falsely detected an error");

            flip_bit(&mut hamming_code.parity_bits[0]);
            assert!(hamming_code.check(), "failed at detecting an error");

            flip_bit(&mut hamming_code.parity_bits[1]);
            assert!(hamming_code.check(), "failed at detecting an error");

            flip_bit(&mut hamming_code.parity_bits[2]);
            assert!(hamming_code.check(), "failed at detecting an error");
        }
    }

    #[test]
    fn error_correcting() {
        let mut hamming_code = HammingCode::new([0,0,1,1]);

        //1 Fehler Korrigieren
        for bits in All4Bits::new() {
            let mut hamming_code = HammingCode::new(bits);
            assert!(!hamming_code.check());
            assert_eq!(0, hamming_code.correct());

            flip_bit(&mut hamming_code.data_bits[0]);
            assert_eq!(1, hamming_code.correct());
            assert_eq!(bits, hamming_code.data_bits);

            flip_bit(&mut hamming_code.data_bits[1]);
            assert_eq!(1, hamming_code.correct());
            assert_eq!(bits, hamming_code.data_bits);

            flip_bit(&mut hamming_code.data_bits[2]);
            assert_eq!(1, hamming_code.correct());
            assert_eq!(bits, hamming_code.data_bits);

            flip_bit(&mut hamming_code.data_bits[3]);
            assert_eq!(1, hamming_code.correct());
            assert_eq!(bits, hamming_code.data_bits);
        }

        //2 Fehler fehlerhaft korrigieren
        hamming_code.data_bits = [0,0,0,0];
        assert!(hamming_code.check());
        assert_eq!(1, hamming_code.correct());
        assert_ne!([0,0,1,1], hamming_code.data_bits);
    }

    #[test]
    fn from_u8_testing() {
        let n: u8 = 1;
        let hamming = HammingCode::from_u8(&n);
        assert_eq!([1,0,0,0], hamming[0].data_bits);
        assert_eq!([0,0,0,0], hamming[1].data_bits);
        
        let n: u8 = u8::MAX;
        let hamming = HammingCode::from_u8(&n);
        assert_eq!([1,1,1,1], hamming[0].data_bits);
        assert_eq!([1,1,1,1], hamming[1].data_bits);
        
        let n: u8 = 0;
        let hamming = HammingCode::from_u8(&n);
        assert_eq!([0,0,0,0], hamming[0].data_bits);
        assert_eq!([0,0,0,0], hamming[1].data_bits);
        
        let n: u8 = 167;
        let hamming = HammingCode::from_u8(&n);
        assert_eq!([1,1,1,0], hamming[0].data_bits);
        assert_eq!([0,1,0,1], hamming[1].data_bits);

        
        
    }

    #[test]
    fn to_u8_test() {
        let hamming = HammingCode { data_bits: [1,1,1,1], parity_bits:[1,1,1] };
        assert_eq!(127, hamming.to_u8());

        let hamming = HammingCode { data_bits: [0,0,0,0], parity_bits:[0,0,0] };
        assert_eq!(0, hamming.to_u8());

        let hamming = HammingCode { data_bits: [1,0,1,1], parity_bits:[1,0,1] };
        assert_eq!(109, hamming.to_u8());

        let hamming = HammingCode { data_bits: [0,0,1,0], parity_bits:[1,1,1] };
        assert_eq!(43, hamming.to_u8());
    }

    #[test]
    fn hamming_test() {
        //panic!("Debug: Es gibt mehr Fehler als es eigentlich dürfte Problem wahrscheinlich bei from_u8 oder from_hamming_u8");
        let bytes: Vec<u8> = vec![ //Hello Wordl as bytes
            72, //H
            101, //e
            108, //l
            108, //l
            111, //o
            32, // SPACE
            87, //w
            111, //o
            114, //r
            108, //l
            100, //d
            32, // SPACE
        ];
        let mut hamming_bytes = hamming_encode(&bytes);

        let mut iter = hamming_bytes.iter();
        let mut i = 0;
        for n in 0..12 {
            let byte1 = HammingCode::from_hamming_u8(iter.next().unwrap());
            let byte2 = HammingCode::from_hamming_u8(iter.next().unwrap());
            println!("{:?}{:?} {:08b}", byte2.data_bits, byte1.data_bits, bytes[i]);
            i += 1;
        }

        flip_n_bit(&mut hamming_bytes[0], 1);
        flip_n_bit(&mut hamming_bytes[3], 6);
        flip_n_bit(&mut hamming_bytes[8], 1);

        assert_eq!(hamming_decode(&hamming_bytes), (bytes, 3));
    }
}