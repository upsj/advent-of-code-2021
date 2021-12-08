use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let _digits: [u8; 10] = [
        0b1110111, // 0
        0b0100100, // 1
        0b1011101, // 2
        0b1101101, // 3
        0b0101110, // 4
        0b1101011, // 5
        0b1111011, // 6
        0b0100101, // 7
        0b1111111, // 8
        0b1101111, // 9
    ];
    let result: Vec<(Vec<u8>, Vec<u8>)> = read_lines("input")
        .unwrap()
        .map(|l| {
            let l2 = l.unwrap();
            let mut it = l2.split("|");
            let sample = it.next().unwrap();
            let output = it.next().unwrap();
            let digits =
                identify_digits(sample.split_whitespace().map(|s| str_to_bits(s)).collect());
            let output_digits = map_digits(
                output.split_whitespace().map(|s| str_to_bits(s)).collect(),
                &digits,
            );
            (digits, output_digits)
        })
        .collect();
    for number in &result {
        println!(
            "{:?} | {:?}",
            number.0.iter().map(bits_to_str).collect::<Vec<_>>(),
            number.1
        );
    }
    println!(
        "{}",
        result
            .iter()
            .map(|ds| ds
                .1
                .iter()
                .map(|v| match v {
                    1 | 4 | 7 | 8 => 1,
                    _ => 0,
                })
                .sum::<i32>())
            .sum::<i32>()
    );
    println!(
        "{}",
        result
            .iter()
            .map(|ds| ds.1.iter().fold(0, |a, b| a * 10 + *b as i32))
            .sum::<i32>()
    );
}

fn remove_digit<T: Fn(u8) -> bool>(d: &mut Vec<u8>, filter: T) -> u8 {
    assert!(d.iter().filter(|v| filter(**v)).count() == 1);
    return d.remove(d.iter().position(|v| filter(*v)).unwrap());
}

fn identify_digits(mut d: Vec<u8>) -> Vec<u8> {
    d.sort_by_key(|v| v.count_ones());
    let d1 = d.remove(0); // 2 segments
    let d7 = d.remove(0); // 3 segments
    let d4 = d.remove(0); // 4 segments
    let d8 = d.remove(d.len() - 1); // 7 segments
    let d9 = remove_digit(&mut d, |v| (v & d4).count_ones() == 4);
    let d2 = remove_digit(&mut d, |v| {
        (v & d1).count_ones() == 1 && (v & d4).count_ones() == 2
    });
    let d5 = remove_digit(&mut d, |v| (v & d2).count_ones() == 3);
    let d6 = remove_digit(&mut d, |v| (v & d1).count_ones() == 1);
    let d3 = remove_digit(&mut d, |v| (v & d8).count_ones() == 5);
    let d0 = d.remove(0);
    [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9].to_vec()
}

fn map_digits(input: Vec<u8>, digits: &Vec<u8>) -> Vec<u8> {
    input
        .into_iter()
        .map(|d| digits.iter().position(|v| *v == d).unwrap() as u8)
        .collect()
}

fn bits_to_str(b: &u8) -> String {
    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut result = String::new();
    for i in 0..7 {
        if *b & (1 << i) != 0 {
            result.push(chars[i]);
        }
    }
    result
}

fn str_to_bits(s: &str) -> u8 {
    let mut result = 0;
    for c in s.chars() {
        let local: u8 = match c {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => panic!("Unknown signal"),
        };
        assert!(local & result == 0);
        result |= local;
    }
    result
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
