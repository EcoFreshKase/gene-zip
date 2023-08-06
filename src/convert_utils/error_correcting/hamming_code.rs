/*
contains all functions to encode and decode bytes with the (7,4) hamming code
*/
use super::rustyham;

/// implements a (7,4) hamming code for the given bytes
/// 
/// returns an error if the conversion from base 10 to binary is faulty
pub fn hamming_encode(bytes: &[u8]) -> Result<Vec<u8>, String> {

    let string_bytes: String = bytes.iter().map(|byte| format!("{:08b}", byte)).collect(); //bytes as string
    let hamming_bits: String  = rustyham::hamming(rustyham::Hamming::EncodeBinary, string_bytes); //bytes in hamming code
    vec_from_string(hamming_bits)
}

/// decodes the given bytes that implement a (7,4) hamming code and corrects all possible errors
/// 
/// The hamming code can neither correctly detect errors or correct errors if more than 1 error occurs every 8 bits (!!!)
/// therefore the result is not guaranteed to be correct.
/// 
/// returns an error if the conversion from base 10 to binary is faulty
pub fn hamming_decode(bytes: &[u8]) -> Result<Vec<u8>, String> {

    let string_bytes: String = bytes.iter().map(|byte| format!("{:08b}", byte)).collect(); //bytes as string
    let decoded_bits: String = rustyham::hamming(rustyham::Hamming::DecodeBinary, string_bytes); //Decode hamming
    vec_from_string(decoded_bits)
}

/// gets a string representing bits and returns a Vec containing all bytes of the bits
/// 
/// returns error if String contains invalid characters
fn vec_from_string(string: String) -> Result<Vec<u8>, String> {
    let chunks = string.as_bytes().chunks(8); //split string into bytes

    let mut output: Vec<u8> = vec![];
    for chunk in chunks {
        let mut byte = 0;
        for (index, bit) in chunk.iter().rev().enumerate() {
            let bit = match bit {
                48 => 0, //ASCII 0
                49 => 1, //ASCII 1
                _ => return Err(format!("Error while creating binary vec from string: invalid ASCII character found: {} ", bit))
            };
            byte += bit * 2u8.pow(index as u32);
        }
        output.push(byte);
    }
    Ok(output)
}

#[cfg(test)]
mod test {
    use crate::convert_utils::error_correcting::rustyham::{hamming, Hamming};
    use super::{hamming_decode, hamming_encode};

    //flips the n-th bit of a byte
    fn flip_n_bit(bit: &mut u8, n: usize) -> u8 {
        let output = if (*bit >> n) & 1 == 1 { //checks if bit is 1
            let rev_bit = !*bit;
            !(rev_bit | (1 << n))
        } else {
            *bit | (1 << n)
        };
        *bit = output;
        output
    }

    #[test]
    fn hamming_test() {
        let bytes: Vec<u8> = vec![ //Hello World as bytes
            72, //H
            101, //e
            108, //l
            108, //l
            111, //o
            32, //SPACE
            87, //w
            111, //o
            114, //r
            108, //l
            100, //d
            32, //SPACE
        ];
        assert_eq!(hamming(Hamming::Decode, hamming(Hamming::Encode, "Hello World".to_string())), "Hello World");


        let hamming_bytes = hamming_encode(&bytes).unwrap();

        //every possible bit flip test
        for (index, _) in hamming_bytes.iter().enumerate() {
            for i in 0..8 {
                let mut hamming_bytes_clone = hamming_bytes.clone();
                flip_n_bit(&mut hamming_bytes_clone[index], i);
                flip_n_bit(&mut hamming_bytes_clone[index], i);
                assert_eq!(hamming_decode(&hamming_bytes).unwrap(), bytes);
            }
        }

    }
}