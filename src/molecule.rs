use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Molecule {
    pub idx: i32,
    pub z_value: Vec<i32>,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
}

impl Molecule {
    pub fn reading_file(&mut self) {
        let data = File::open("sample.dat").unwrap_or_else(|error| panic!("ERROR: {}", error));
        let buf_reader = BufReader::new(data);
        let mut lines = buf_reader.lines();
        lines.next();

        for l in lines {
            if let Ok(ip) = l {
                let vec = ip.split_whitespace().collect::<Vec<&str>>();
                &self.z_value.push(vec[0].parse::<i32>().unwrap());
                &self.x.push(vec[1].parse::<f32>().unwrap());
                &self.y.push(vec[2].parse::<f32>().unwrap());
                &self.z.push(vec[3].parse::<f32>().unwrap());
            }
        }
    }
}

fn find_bond_len(atom1: &Molecule, atom2: &Molecule) -> f32 {
    let r: f32 = 0.0;
    r
}
