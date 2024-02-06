use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn reading_file() {
    let mut z_value = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let data = File::open("sample.dat").unwrap_or_else(|error| panic!("ERROR: {}", error));
    let mut buf_reader = BufReader::new(data);
    for l in buf_reader.lines() {
        l.unwrap();
    }
}

fn main() {
    reading_file();
}
