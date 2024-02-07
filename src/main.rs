use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn reading_file() {
    let mut z_value = Vec::<i32>::new();
    let mut x = Vec::<f32>::new();
    let mut y = Vec::<f32>::new();
    let mut z = Vec::<f32>::new();

    let data = File::open("sample.dat").unwrap_or_else(|error| panic!("ERROR: {}", error));
    let buf_reader = BufReader::new(data);
    let mut lines = buf_reader.lines();
    lines.next();

    for l in lines {
        if let Ok(ip) = l {
            let vec = ip.split_whitespace().collect::<Vec<&str>>();
            z_value.push(vec[0].parse::<i32>().unwrap());
            x.push(vec[1].parse::<f32>().unwrap());
            y.push(vec[2].parse::<f32>().unwrap());
            z.push(vec[3].parse::<f32>().unwrap());
        }
    }

    println!("{:?}", z_value);
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}

fn main() {
    reading_file();
}
