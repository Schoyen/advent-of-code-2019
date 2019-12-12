struct Object {
    name: String,
    num_orbits: u32,
}

impl Object {
    fn new(name: String) -> Object {
        Object {
            name,
            num_orbits: 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
