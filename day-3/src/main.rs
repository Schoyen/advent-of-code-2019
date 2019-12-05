use std::{fs, path::Path};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Point) -> Point {
        self.add(&Point::new(-other.x, -other.y))
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

//#[derive(Debug)]
//struct Line {
//    start: Point,
//    end: Point,
//    is_horizontal: bool,
//}
//
//impl Line {
//    fn new(x_0: i32, y_0: i32, x_1: i32, y_1: i32) -> Line {
//        Line {
//            start: Point::new(x_0, y_0),
//            end: Point::new(x_1, y_1),
//            is_horizontal: (y_1 - y_0) == 0,
//        }
//    }
//
//    fn point_of_intersection(&self, other: &Line) -> Option<Point> {
//        Some(Point::new(0, 0))
//    }
//}

fn parse_line(line_str: &str) -> Vec<Point> {
    let cursor = Point::new(0, 0);
    let mut line: Vec<Point> = Vec::new();
    line.push(cursor);

    let moves = line_str.split(",");

    for _move in moves {
        let direction = _move.chars().nth(0).unwrap();

        let x_move = if direction == 'R' {
            1
        } else if direction == 'L' {
            -1
        } else {
            0
        };

        let y_move = if direction == 'U' {
            1
        } else if direction == 'D' {
            -1
        } else {
            0
        };

        let move_direction = Point::new(x_move, y_move);
        let num_steps = &_move[1..].parse::<i32>().unwrap();

        for _i in 0..*num_steps {
            line.push(line[line.len() - 1].add(&move_direction));
        }
    }

    line
}

fn read_file() -> Vec<Vec<Point>> {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let lines_text = contents.split("\n");
    let mut lines = Vec::new();

    for line in lines_text {
        lines.push(parse_line(line));
    }

    lines
}

fn main() {
    let mut lines = read_file();

    let mut line_1 = lines.remove(0);
    let mut line_2 = lines.remove(0);

    //line_1.sort_by(|a, b| (a.x.abs() + a.y.abs()).cmp(&(b.x.abs() + b.y.abs())));
    //line_2.sort_by(|a, b| (a.x.abs() + a.y.abs()).cmp(&(b.x.abs() + b.y.abs())));

    line_1.sort_by(|a, b| a.manhattan_distance().cmp(&b.manhattan_distance()));
    line_2.sort_by(|a, b| a.manhattan_distance().cmp(&b.manhattan_distance()));

    let mut closest = -1;

    for i in 1..line_1.len() {
        let p_1 = &line_1[i];
        for j in 1..line_2.len() {
            let p_2 = &line_2[j];

            if p_1.sub(&p_2).manhattan_distance() == 0 {
                closest = p_1.manhattan_distance();
                break;
            }
        }
        if closest > -1 {
            break;
        }
    }
    println!("Part 1: {}", closest);
}
