use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let numbers: Vec<_> = read_lines("input")
        .unwrap()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect();
    println!(
        "increasing: {}",
        numbers
            .iter()
            .zip(numbers.iter().skip(1))
            .filter(|(a, b)| b > a)
            .count()
    );
    let windowed: Vec<_> = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .zip(numbers.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();
    println!(
        "3-windows: {}",
        windowed
            .iter()
            .zip(windowed.iter().skip(1))
            .filter(|(a, b)| b > a)
            .count()
    )
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
