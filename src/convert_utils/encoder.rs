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
///Error might occour if an buf_read couldn't read from the file or if the binary sequence couldn't be converted to DNA.
///Every file can be converted to DNA but an error is returned when the buf_read returns a string that not only contains 1 and 0.
pub fn easy_encode(bytes: Vec<u8>) -> Result<String, String> {
    let mut output = String::new();
    for byte in bytes {
        //split byte into 2 bit chunks
        let douple_bits = format!("{:08b}", byte).chars() 
            .collect::<Vec<char>>();

        for douple_bit in douple_bits.chunks(2) {
            //match every douple bit to nucleotide
            match &douple_bit.iter().collect::<String>()[..] {
                "00" => output.push('A'),
                "01" => output.push('T'),
                "10" => output.push('C'),
                "11" => output.push('G'),
                s @ _ => return Err(format!("found invalid string while machting bits: \"{}\"", s).to_string())
            };
        }
    }

    Ok(output)
}