use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut pos1 = 0;
    let mut depth1 = 0;
    let mut pos2 = 0;
    let mut depth2 = 0;
    let mut aim2 = 0;
    for line in read_lines("input").unwrap() {
        let line = line.unwrap();
        let mut it = line.split_whitespace();
        let action = it.next().unwrap();
        let number = it.next().unwrap().parse::<i32>().unwrap();
        // part 1
        match action {
            "forward" => pos1 += number,
            "down" => depth1 += number,
            "up" => depth1 -= number,
            _ => panic!("Unknown action"),
        }
        // part 2
        match action {
            "forward" => {
                pos2 += number;
                depth2 += number * aim2
            }
            "down" => aim2 += number,
            "up" => aim2 -= number,
            _ => panic!("Unknown action"),
        }
    }
    println!("part 1: {} {} {}", pos1, depth1, pos1 * depth1);
    println!("part 2: {} {} {}", pos2, depth2, pos2 * depth2);
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
