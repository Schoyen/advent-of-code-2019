use std::{fs, path::Path};

fn do_op(opcodes: &mut Vec<usize>, pos: usize) {
    let arg_left = opcodes[opcodes[pos + 1]];
    let arg_right = opcodes[opcodes[pos + 2]];
    let res_pos = opcodes[pos + 3];

    if opcodes[pos] == 1 {
        opcodes[res_pos] = arg_left + arg_right;
    } else {
        opcodes[res_pos] = arg_left * arg_right;
    }
}

fn execute_part_1(opcodes: &mut Vec<usize>) -> usize {
    let mut i = 0;
    while i < opcodes.len() {
        if opcodes[i] == 1 || opcodes[i] == 2 {
            do_op(opcodes, i);
            i += 3;
        } else if opcodes[i] == 99 {
            break;
        } else {
            dbg!(i);
            panic!("Oi, doing!");
        }
        i += 1;
    }
    opcodes[0]
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split(",");
    let mut opcodes = Vec::new();

    for s in split {
        opcodes.push(s.parse::<usize>().unwrap());
    }

    // Part 1 specification
    opcodes[1] = 12;
    opcodes[2] = 2;

    println!("Part 1: {}", execute_part_1(&mut opcodes));
}
