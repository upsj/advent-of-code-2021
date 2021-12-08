use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut numbers: Vec<i32> = read_lines("input")
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    numbers.sort();
    let median = numbers[numbers.len() / 2];
    let fuel = numbers.iter().map(|x| (x - median).abs()).fold(0, |a,b| a+b);
    let fuel2 = |m: &i32| numbers.iter().map(|x| (x - m).abs() * ((x - m).abs() + 1) / 2).fold(0, |a,b| a+b);
    let min = numbers[0];
    let max = numbers[numbers.len() - 1];
    let median2 = (min..=max).min_by_key(fuel2).unwrap();
    println!("{} {}", median, fuel);
    println!("{} {}", median2, fuel2(&median2));
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
