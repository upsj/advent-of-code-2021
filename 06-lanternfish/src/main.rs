use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num_bigint::{BigInt, Sign};

fn main() {
    let numbers: Vec<i32> = read_lines("input")
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut counts = vec![BigInt::new(Sign::NoSign, Vec::new()); 9];
    for number in numbers {
        counts[number as usize] += 1;
    }
    for _day in 0..256 {
        // shift everything to the front
        let parents = counts.remove(0);
        // reset parents to state 6
        counts[6] += &parents;
        // add children to state 8
        counts.push(parents);
    }
    println!("{}", counts.iter().fold(BigInt::new(Sign::NoSign, Vec::new()), |a,b| a + b));
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
