use std::{fs, path::Path};

fn read_file(filename: &Path) -> Vec<i32> {
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split(",");
    let mut memory = Vec::new();

    for s in split {
        memory.push(s.parse::<i32>().unwrap());
    }

    memory
}

fn parameter_modes(mem: i32) -> Vec<u8> {
    let A = ((mem / 10000) % 2) as u8;
    let B = ((mem / 1000) % 2) as u8;
    let C = ((mem / 100) % 2) as u8;

    vec![A, B, C]
}

fn binary_op(memory: &mut Vec<i32>, eip: &mut usize, op: u8) -> i8 {
    let param_modes = parameter_modes(memory[*eip]);

    let a = match param_modes[2] {
        0 => memory[memory[*eip + 1] as usize],
        1 => memory[*eip + 1],
        _ => panic!("Invalid parameter mode {}", param_modes[2]),
    };

    let b = match param_modes[1] {
        0 => memory[memory[*eip + 2] as usize],
        1 => memory[*eip + 2],
        _ => panic!("Invalid parameter mode {}", param_modes[1]),
    };

    let res_pos = memory[*eip + 3] as usize;

    memory[res_pos] = match op {
        0 => a + b,
        1 => a * b,
        2 => {
            if a < b {
                1
            } else {
                0
            }
        }
        3 => {
            if a == b {
                1
            } else {
                0
            }
        }
        _ => panic!("Invalid operation {}", op),
    };

    *eip += 4;
    1
}

fn op_01(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    binary_op(memory, eip, 0)
}

fn op_02(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    binary_op(memory, eip, 1)
}

fn op_07(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    binary_op(memory, eip, 2)
}

fn op_08(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    binary_op(memory, eip, 3)
}

fn op_03(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    let res_pos = memory[*eip + 1] as usize;

    println!("Op 3 input: ");

    let mut instruction = String::new();
    std::io::stdin().read_line(&mut instruction);
    memory[res_pos] = instruction.trim_end().to_string().parse::<i32>().unwrap();

    *eip += 2;
    1
}

fn op_04(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    let res_pos = memory[*eip + 1] as usize;

    println!("Op 4 output: {}", memory[res_pos]);

    *eip += 2;
    1
}

fn jmp_op(memory: &mut Vec<i32>, eip: &mut usize, op: u8) -> i8 {
    let param_modes = parameter_modes(memory[*eip]);

    let a = match param_modes[2] {
        0 => memory[memory[*eip + 1] as usize],
        1 => memory[*eip + 1],
        _ => panic!("Invalid parameter mode {}", param_modes[2]),
    };

    let b = match param_modes[1] {
        0 => memory[memory[*eip + 2] as usize],
        1 => memory[*eip + 2],
        _ => panic!("Invalid parameter mode {}", param_modes[1]),
    };

    *eip = match op {
        // jump-if-true
        0 => {
            if a != 0 {
                b as usize
            } else {
                *eip + 3
            }
        }
        // jump-if-false
        1 => {
            if a == 0 {
                b as usize
            } else {
                *eip + 3
            }
        }
        _ => panic!("Invalid jump operation {}", op),
    };

    1
}

fn op_05(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    jmp_op(memory, eip, 0)
}

fn op_06(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    jmp_op(memory, eip, 1)
}

fn do_op(memory: &mut Vec<i32>, eip: &mut usize) -> i8 {
    let mem = memory[*eip];
    let op: u8 = (mem % 100) as u8;

    match op {
        99 => return -1,
        1 => return op_01(memory, eip),
        2 => return op_02(memory, eip),
        3 => return op_03(memory, eip),
        4 => return op_04(memory, eip),
        5 => return op_05(memory, eip),
        6 => return op_06(memory, eip),
        7 => return op_07(memory, eip),
        8 => return op_08(memory, eip),
        _ => panic!("Invalid opcode: {}", op),
    }
}

fn execute_part_1(memory: &mut Vec<i32>) -> i32 {
    let mut eip: usize = 0;

    while do_op(memory, &mut eip) > 0 {}

    memory[0]
}

fn main() {
    let mut memory = read_file(Path::new("./dat/input.dat"));

    println!("Part 1: {}", execute_part_1(&mut memory));
}
