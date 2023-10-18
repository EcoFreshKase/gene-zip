use std::{fs::read, path::Path};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gene_zip::convert_utils::easy_encode;

const small_file: &str = "benches\\test_files\\small_file.txt";
const medium_file: &str = "benches\\test_files\\medium_file.txt";

fn get_vec_from_file(path: &str) -> Vec<u8> {
    match read(path) {
        Ok(n) => n,
        Err(e) => {
            panic!("{}", format!("error: {}; path: {}", e, path))
        },
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("easy_encode", |b| b.iter(|| easy_encode(black_box(get_vec_from_file(small_file)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);