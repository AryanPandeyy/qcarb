use crate::molecule::Molecule;
pub mod molecule;

fn main() {
    let mut atom = Molecule {
        idx: 7,
        z_value: Vec::<i32>::new(),
        x: Vec::<f32>::new(),
        y: Vec::<f32>::new(),
        z: Vec::<f32>::new(),
    };
    atom.reading_file();
    atom.find_bond_len();
}
