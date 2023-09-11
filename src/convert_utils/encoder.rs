/*
contains functions to translate binary to DNA sequences
*/

///translates binary to DNA with the following pattern:
/// 00 -> A
/// 01 -> T
/// 10 -> C
/// 11 -> G
///counterpart to the easy_decode function
///
///takes a path to a file as an &str and returns a Result.
///
///Error might occur if an buf_read couldn't read from the file or if the binary sequence couldn't be converted to DNA.
///Every file can be converted to DNA but an error is returned when the buf_read returns a string that not only contains 1 and 0.
pub fn easy_encode(bytes: Vec<u8>) -> Result<String, String> {
    let mut output = String::new();
    for byte in bytes {
        //split byte into 2 bit chunks
        format!("{:08b}", byte).chars() 
            .collect::<Vec<char>>()
            .chunks(2)
            .for_each(|double_bit| {
                match double_bit {
                    //match every double bit to nucleotide
                    ['0', '0'] => output.push('A'),
                    ['0', '1'] => output.push('T'),
                    ['1', '0'] => output.push('C'),
                    ['1', '1'] => output.push('G'),
                    _ => (),
                }
            });
    }

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::easy_encode;
    #[test]
    fn easy_encode_test() {
        // #1: 00 00 00 00
        // #2: 11 11 11 11
        // #3: 00 00 11 00
        // #4: 01 00 00 01
        //
        // expected result: 
        //  A A A A 
        //  G G G G 
        //  A A G A 
        //  T A A T 
        let chars = vec![0, u8::MAX, 12, 65];
        let string_encoded = easy_encode(chars);
        let expected_result = "AAAAGGGGAAGATAAT";

        assert_eq!(string_encoded.unwrap(), expected_result);
    }
}