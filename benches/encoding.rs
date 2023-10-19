use std::fs::{read, read_to_string};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gene_zip::convert_utils::{easy_encode, easy_decode};

const SMALL_FILE: &str = "benches\\test_files\\small_file.txt";
const SMALL_DNA_FILE: &str = "benches\\test_files\\small_file.fasta";

fn get_vec_from_file(path: &str) -> Vec<u8> {
    match read(path) {
        Ok(n) => n,
        Err(e) => {
            panic!("{}", format!("error: {}; path: {}", e, path))
        },
    }
}

fn get_dna_from_file(path: &str) -> String {
    match read_to_string(path) {
        Ok(n) => {
            let index = match n.find('\n') { //end of first line
                Some(n) => n,
                None => panic!("File fault: it does not follow the fasta format "),
            };
            n[index+1..].replace('\n', "")
        },
        Err(_) => panic!("error while reading file. Please try again."), 
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "easy_encode small file", 
        |b| {
            b.iter(|| easy_encode(black_box(get_vec_from_file(SMALL_FILE))))
        }
    );

    c.bench_function(
        "easy_decode small file", 
        |b| {
            b.iter(|| easy_decode(black_box(&get_dna_from_file(SMALL_DNA_FILE))))
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);