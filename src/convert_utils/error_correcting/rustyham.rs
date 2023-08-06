/*
Hamming Code Implementation from tckmn
Github-repo: https://github.com/tckmn/rustyham
*/

use std::iter::repeat;

#[allow(dead_code)]
pub enum Hamming { Encode, Decode, EncodeBinary, DecodeBinary }

pub fn hamming(action: Hamming, s: String) -> String {
    match action {
        Hamming::Encode | Hamming::EncodeBinary => {
            // get an iterator over the individual bits
            let bytes;
            let bytes_str = match action {
                Hamming::EncodeBinary => s + "1",  // the final 1 signifies EOS
                _ => {
                    // convert ASCII string to binary
                    bytes = s.into_bytes();  // takes ownership of s
                    bytes.iter().map(|&c| format!("{:0>1$b}", c, 7))
                        .collect::<Vec<String>>().concat()
                }
            };
            let mut bytes_iter = bytes_str.chars();
            // we should assume that ownership of s has already been
            // transferred away by this point

            // compute block and message length
            let mlen = bytes_str.len() as u32;
            let lenpow = (2..).find(|&r| 2u32.pow(r) - r > mlen).unwrap();
            let len = 2usize.pow(lenpow) - 1;

            // the thing we're storing the hamming code in
            let mut code: Vec<bool> = repeat(false).take(len).collect();

            // set data bits
            for i in 1..len {
                if (i & (i - 1)) != 0 {  // if i is not a power of 2
                    code[i-1] = bytes_iter.next().unwrap_or('0') == '1';
                }
            }

            // set parity bits
            for i in 0..lenpow {
                code[2usize.pow(i) - 1] = calc_parity(&code, i);
            }

            code.into_iter().map(|x| if x {"1"} else {"0"})
                .collect::<Vec<_>>().concat()
        },
        Hamming::Decode | Hamming::DecodeBinary => {
            // verify parity bits, fix 1-bit-flipped errors if any
            let len = s.len();
            let lenpow = ((len + 1) as f32).sqrt().round() as u32;
            let mut chars = s.chars().map(|x| x == '1').collect::<Vec<bool>>();
            let mut flipped_bit = -1i32;
            while (0..lenpow).any(|i| calc_parity(&chars, i)) {
                if flipped_bit != -1 {
                    chars[flipped_bit as usize] = !chars[flipped_bit as usize];
                }
                flipped_bit += 1;
                chars[flipped_bit as usize] = !chars[flipped_bit as usize];
            }

            // collect all bits at non-powers-of-2
            let data = chars.iter().enumerate()
                .filter(|x| ((x.0 + 1) & x.0) != 0)
                .map(|x| if *x.1 {'1'} else {'0'});

            // return
            match action {
                Hamming::DecodeBinary => {
                    // have to chop off everything from the final 1
                    let data_str = data.collect::<String>();
                    let idx = (data_str[..]).rfind('1').unwrap();
                    (data_str[..idx]).to_string()
                },
                _ => {
                    // we have to chop off the 0 padding if it exists
                    let cslice = &data.collect::<Vec<char>>()[..];
                    let mut chunks = cslice.chunks(7).map(|x| {
                        x.iter().cloned().collect::<String>()
                    }).collect::<Vec<String>>();
                    if chunks[chunks.len()-1].len() < 7 { chunks.pop(); }
                    while chunks[chunks.len()-1] == "0000000" { chunks.pop(); }

                    let chars = chunks.iter()
                        .map(|x| u8::from_str_radix(&x[..], 2).unwrap())
                        .collect::<Vec<u8>>();
                    String::from_utf8(chars).unwrap()
                }
            }
        }
    }
}

fn calc_parity(code: &[bool], i: u32) -> bool {
    let bi = 2usize.pow(i) - 1;
    let (mut parity, mut ignore, mut counter) = (false, false, 0);
    for bit in code.iter().skip(bi) {
        if !ignore && *bit { parity = !parity }
        counter += 1;
        if counter >= 2u32.pow(i) {
            ignore = !ignore;
            counter = 0;
        }
    }
    parity
}