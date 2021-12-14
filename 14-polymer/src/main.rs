use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut line_iter = read_lines("input").unwrap().map(|s| s.unwrap());
    let polymer: Vec<char> = line_iter.next().unwrap().as_str().chars().collect();
    let productions: HashMap<(char, char), char> = line_iter
        .skip_while(|s| s.is_empty())
        .map(|s| {
            let parts: [&str; 2] = s.split(" -> ").collect::<Vec<&str>>().try_into().unwrap();
            (
                (
                    parts[0].chars().nth(0).unwrap(),
                    parts[0].chars().nth(1).unwrap(),
                ),
                parts[1].chars().next().unwrap(),
            )
        })
        .collect();
    // remember the last char, because that one will be at an odd position in the end
    let last_char = *polymer.last().unwrap();
    // store pairs of chars + whether they are at a even or odd location
    let mut pairs: HashMap<(char, char, bool), usize> = polymer
        .iter()
        .zip(polymer.iter().skip(1))
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, (a, b))| {
            *map.entry((*a, *b, i % 2 == 0)).or_insert(0 as usize) += 1;
            map
        });
    for _i in 0..40 {
        pairs = pairs
            .into_iter()
            .flat_map(|((a, b, _), c)| {
                let m = productions[&(a, b)];
                [((a, m, true), c), ((m, b, false), c)].into_iter()
            })
            .fold(HashMap::new(), |mut map, (pair, c)| {
                *map.entry(pair).or_insert(0 as usize) += c;
                map
            });
    }
    let mut counts: Vec<(char, usize)> = pairs
        .into_iter()
        .filter(|((_, _, even), _)| *even)
        .flat_map(|((a, b, _), c)| [(a, c), (b, c)])
        .chain([(last_char, 1)].into_iter()) // insert the last char again
        .fold(HashMap::new(), |mut map, (c, count)| {
            *map.entry(c).or_insert(0 as usize) += count;
            map
        })
        .into_iter()
        .collect();
    counts.sort_by_key(|(_, count)| *count);
    println!("{}", counts[counts.len() - 1].1 - counts[0].1);
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
