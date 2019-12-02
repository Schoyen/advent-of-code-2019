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

fn read_file() -> Vec<usize> {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split(",");
    let mut opcodes = Vec::new();

    for s in split {
        opcodes.push(s.parse::<usize>().unwrap());
    }

    opcodes
}

fn main() {
    let mut opcodes = read_file();

    // Part 1 specification
    opcodes[1] = 12;
    opcodes[2] = 2;

    println!("Part 1: {}", execute_part_1(&mut opcodes));

    // Part 2
    let mut noun_out = 0;
    let mut verb_out = 0;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut opcodes = read_file();
            opcodes[1] = noun;
            opcodes[2] = verb;

            let res = execute_part_1(&mut opcodes);

            if res == 19690720 {
                noun_out = noun;
                verb_out = verb;
                break;
            }
        }
    }

    println!("Part 2: {}", 100 * noun_out + verb_out);
}
