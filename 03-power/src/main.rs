use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines: Vec<String> = read_lines("input").unwrap().map(|s| s.unwrap()).collect();
    let num_lines = lines.len() as i32;
    let counters = count_bits(&lines);
    let (gamma, epsilon) = compute_gamma_epsilon(&counters, num_lines);
    let gamma_val = binary_to_decimal(&gamma);
    let epsilon_val = binary_to_decimal(&epsilon);
    println!(
        "Power: {} {} {}",
        gamma_val,
        epsilon_val,
        gamma_val * epsilon_val
    );
    let mut o2_lines: Vec<String> = lines
        .to_owned()
        .into_iter()
        .filter(|l| {
            l.as_str().chars().next().unwrap()
                == (if 2 * counters[0] > num_lines {
                    '0'
                } else {
                    '1'
                })
        })
        .collect();
    println!("{:?}", counters);
    let mut o2_pos = 1;
    while o2_lines.len() > 1 {
        let local_counters = count_bits(&o2_lines);
        let local_lines = o2_lines.len() as i32;
        o2_lines = o2_lines
            .into_iter()
            .filter(|l| {
                l.as_str().chars().nth(o2_pos).unwrap()
                    == (if 2 * local_counters[o2_pos] > local_lines {
                        '0'
                    } else {
                        '1'
                    })
            })
            .collect();
        o2_pos += 1;
    }
    let mut co2_lines: Vec<String> = lines
        .into_iter()
        .filter(|l| {
            l.as_str().chars().next().unwrap()
                == (if 2 * counters[0] <= num_lines {
                    '0'
                } else {
                    '1'
                })
        })
        .collect();
    let mut co2_pos = 1;
    while co2_lines.len() > 1 {
        let local_counters = count_bits(&co2_lines);
        let local_lines = co2_lines.len() as i32;
        co2_lines = co2_lines
            .into_iter()
            .filter(|l| {
                l.as_str().chars().nth(co2_pos).unwrap()
                    == (if 2 * local_counters[co2_pos] <= local_lines {
                        '0'
                    } else {
                        '1'
                    })
            })
            .collect();
        println!("{:?}", co2_lines);
        co2_pos += 1;
    }
    let o2_binary = o2_lines[0].as_str().chars().collect();
    let co2_binary = co2_lines[0].as_str().chars().collect();
    let o2_val = binary_to_decimal(&o2_binary);
    let co2_val = binary_to_decimal(&co2_binary);
    println!("Life support: {} {} {}", o2_val, co2_val, o2_val * co2_val);
}

fn count_bits(lines: &Vec<String>) -> Vec<i32> {
    let num_digits = lines[0].len();
    let mut counters = vec![0; num_digits];
    for line in lines {
        for (i, c) in line.as_str().chars().enumerate() {
            match c {
                '0' => counters[i] += 1,
                '1' => (),
                _ => panic!("Unknown char"),
            }
        }
    }
    counters
}

fn compute_gamma_epsilon(counters: &Vec<i32>, num_lines: i32) -> (Vec<char>, Vec<char>) {
    let gamma: Vec<_> = counters
        .iter()
        .map(|v| {
            if 2 * v > num_lines {
                '0'
            } else if 2 * v < num_lines {
                '1'
            } else {
                panic!("Undecidable")
            }
        })
        .collect();
    let epsilon: Vec<_> = gamma
        .iter()
        .map(|v| match v {
            '0' => '1',
            '1' => '0',
            _ => panic!(),
        })
        .collect();
    (gamma, epsilon)
}

fn binary_to_decimal(binary: &Vec<char>) -> i32 {
    binary
        .iter()
        .enumerate()
        .map(|(i, c)| {
            (1 << (binary.len() - 1 - i))
                * match c {
                    '1' => 1,
                    '0' => 0,
                    _ => panic!(),
                }
        })
        .fold(0, |a, b| a + b)
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
