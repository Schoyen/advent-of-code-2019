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

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    is_horizontal: bool,
    num_steps: i32,
}

impl Line {
    fn new(x_0: i32, y_0: i32, x_1: i32, y_1: i32) -> Line {
        Line {
            start: Point::new(x_0, y_0),
            end: Point::new(x_1, y_1),
            is_horizontal: (y_1 - y_0) == 0,
            num_steps: ((y_1 - y_0).abs() + (x_1 - x_0).abs()),
        }
    }

    fn closest_point_of_intersection(&self, other: &Line) -> Option<Point> {
        if self.start.is_origin() && other.start.is_origin() {
            return None;
        }

        if self.is_horizontal == other.is_horizontal {
            // We assume that two lines only intersect once
            return None;
        }

        if self.is_horizontal {
            assert_eq!(self.start.y, self.end.y);
            assert_eq!(other.start.x, other.end.x);

            if !lies_between(other.start.y, other.end.y, self.start.y) {
                return None;
            }

            if lies_between(self.start.x, self.end.x, other.start.x) {
                return Some(Point::new(other.start.x, self.start.y));
            }

            return None;
        } else {
            assert_eq!(self.start.x, self.end.x);
            assert_eq!(other.start.y, other.end.y);

            if !lies_between(other.start.x, other.end.x, self.start.x) {
                return None;
            }

            if lies_between(self.start.y, self.end.y, other.start.y) {
                return Some(Point::new(self.start.x, other.start.y));
            }

            return None;
        }
    }

    fn number_of_steps_to_point(&self, p: &Point) -> i32 {
        assert!(lies_between(self.start.x, self.end.x, p.x));
        assert!(lies_between(self.start.y, self.end.y, p.y));

        p.sub(&self.start).manhattan_distance()
    }
}

fn lies_between(x_0: i32, x_1: i32, p: i32) -> bool {
    let direction = x_1 - x_0;

    assert!(direction.abs() >= 0);

    if direction < 0 {
        return x_1 <= p && p <= x_0;
    }

    return x_0 <= p && p <= x_1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_vertical_lines() {
        let line_1 = Line::new(1, 2, 1, -2);
        let line_2 = Line::new(2, 2, 2, -2);

        assert!(line_1.closest_point_of_intersection(&line_2).is_none());
    }

    #[test]
    fn test_parallel_horizontal_lines() {
        let line_1 = Line::new(2, 1, -2, 1);
        let line_2 = Line::new(2, 2, -2, 2);

        assert!(line_1.closest_point_of_intersection(&line_2).is_none());
    }

    #[test]
    fn test_crossing_1() {
        let line_1 = Line::new(1, 3, 6, 3);
        let line_2 = Line::new(4, -1, 4, 5);

        let intersection = Point::new(4, 3);

        let calc_intersection = line_1.closest_point_of_intersection(&line_2);
        assert!(calc_intersection.is_some());

        assert_eq!(
            line_1.closest_point_of_intersection(&line_2).unwrap(),
            intersection
        );
    }

    #[test]
    fn test_crossing_2() {
        let line_1 = Line::new(10, -3, 6, -3);
        let line_2 = Line::new(7, -1, 7, -5);

        let intersection = Point::new(7, -3);

        let calc_intersection = line_1.closest_point_of_intersection(&line_2);
        assert!(calc_intersection.is_some());

        assert_eq!(
            line_1.closest_point_of_intersection(&line_2).unwrap(),
            intersection
        );
    }
}

fn parse_line_2(line_str: &str) -> Vec<Line> {
    let mut start = Point::new(0, 0);
    let mut lines = Vec::new();

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

        let num_steps = &_move[1..].parse::<i32>().unwrap();
        let end = Point::new(start.x + x_move * num_steps, start.y + y_move * num_steps);
        lines.push(Line::new(start.x, start.y, end.x, end.y));

        start = end;
    }

    lines
}

fn read_file_2(filename: &Path) -> Vec<Vec<Line>> {
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let lines_text = contents.split("\n");
    let mut lines = Vec::new();

    for line in lines_text {
        lines.push(parse_line_2(line));
    }

    lines
}

fn main() {
    let mut lines = read_file_2(Path::new("./dat/input.dat"));

    let line_1 = lines.remove(0);
    let line_2 = lines.remove(0);

    let mut closest = 1000000000;
    let mut shortest_path = 1000000000;

    let mut l_1_steps = 0;
    let mut l_2_steps = 0;

    for i in 0..line_1.len() {
        let l_1 = &line_1[i];

        for j in 0..line_2.len() {
            let l_2 = &line_2[j];

            let intersection = l_1.closest_point_of_intersection(&l_2);

            if intersection.is_some() {
                let p_1 = intersection.unwrap();
                let p_dist = p_1.manhattan_distance();

                if p_dist < closest {
                    closest = p_dist;
                }

                let path = l_1_steps
                    + l_1.number_of_steps_to_point(&p_1)
                    + l_2_steps
                    + l_2.number_of_steps_to_point(&p_1);

                if path < shortest_path {
                    shortest_path = path;
                }
            }
            l_2_steps += l_2.num_steps;
        }
        l_2_steps = 0;
        l_1_steps += l_1.num_steps;
    }

    println!("Part 1: {}", closest);
    println!("Part 2: {}", shortest_path);
}
