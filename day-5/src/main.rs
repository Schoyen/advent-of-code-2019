use std::{fs, path::Path};

fn read_file(filename: &Path) -> Vec<i32> {
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split(",");
    let mut opcodes = Vec::new();

    for s in split {
        opcodes.push(s.parse::<i32>().unwrap());
    }

    opcodes
}

fn main() {
    let mut opcodes = read_file(Path::new("./dat/input.dat"));
}
