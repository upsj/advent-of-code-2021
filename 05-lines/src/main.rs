use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    a: Point,
    b: Point,
}

fn parse_point(point: String) -> Point {
    let mut iter = point.split(",").map(|s| s.parse::<i32>().unwrap());
    let x = iter.next().unwrap();
    let y = iter.next().unwrap();
    Point { x: x, y: y }
}

fn parse_line(line: String) -> Line {
    let mut iter = line.split(" ");
    let a = parse_point(iter.next().unwrap().to_string());
    iter.next();
    let b = parse_point(iter.next().unwrap().to_string());
    Line { a: a, b: b }
}

fn is_ortho(line: &Line) -> bool {
    (line.a.x == line.b.x) || (line.a.y == line.b.y)
}

fn is_diag(line: &Line) -> bool {
    (line.a.x - line.b.x).abs() == (line.a.y - line.b.y).abs()
}

fn main() {
    let lines: Vec<Line> = read_lines("input")
        .unwrap()
        .map(|s| parse_line(s.unwrap()))
        .filter(|l| is_ortho(l) || is_diag(l))
        .collect();
    let mut storage1 = HashSet::new();
    let mut intersections1 = HashSet::new();
    let mut storage2 = HashSet::new();
    let mut intersections2 = HashSet::new();
    for line in &lines {
        if line.a.x == line.b.x {
            let x = line.a.x;
            for y in cmp::min(line.a.y, line.b.y)..=cmp::max(line.a.y, line.b.y) {
                let p = Point { x: x, y: y };
                if !storage1.insert(Point { x: x, y: y }) {
                    intersections1.insert(p);
                }
                if !storage2.insert(Point { x: x, y: y }) {
                    intersections2.insert(p);
                }
            }
        } else if line.a.y == line.b.y {
            let y = line.a.y;
            for x in cmp::min(line.a.x, line.b.x)..=cmp::max(line.a.x, line.b.x) {
                let p = Point { x: x, y: y };
                if !storage1.insert(Point { x: x, y: y }) {
                    intersections1.insert(p);
                }
                if !storage2.insert(p) {
                    intersections2.insert(p);
                }
            }
        } else {
            let len = (line.b.x - line.a.x).abs();
            let dx = (line.b.x - line.a.x).signum();
            let dy = (line.b.y - line.a.y).signum();
            for i in 0..=len {
                let p = Point {
                    x: line.a.x + dx * i,
                    y: line.a.y + dy * i,
                };
                if !storage2.insert(p) {
                    intersections2.insert(p);
                }
            }
        }
    }
    println!("{}", intersections1.len());
    println!("{}", intersections2.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
