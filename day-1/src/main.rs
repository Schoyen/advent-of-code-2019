use std::{fs, path::Path};

fn fuel_counter(weight: i32) -> i32 {
    let mut fuel_weight = (((weight as f64) / (3 as f64)).floor() as i32) - 2;

    if fuel_weight < 0 {
        fuel_weight = 0;
    }

    fuel_weight
}

fn day_1_counter(modules: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for f in modules {
        sum += fuel_counter(*f);
    }

    sum
}

fn day_2_counter(modules: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for f in modules {
        let mut module_fuel = fuel_counter(*f);

        while module_fuel > 0 {
            sum += module_fuel;
            module_fuel = fuel_counter(module_fuel);
        }
    }

    sum
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split("\n");
    let mut module_weights = Vec::new();

    for s in split {
        module_weights.push(s.parse::<i32>().unwrap())
    }

    println!("Day 1, part 1: {}", day_1_counter(&module_weights));
    println!("Day 1, part 2: {}", day_2_counter(&module_weights));
}
