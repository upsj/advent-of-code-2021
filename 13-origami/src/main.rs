use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_point(s: String) -> [i32; 2] {
    s.split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap()
}

struct Fold {
    vertical: bool,
    position: i32,
}

fn fold(points: HashSet<[i32; 2]>, fold: &Fold) -> HashSet<[i32; 2]> {
    points
        .iter()
        .map(|[x, y]| {
            if fold.vertical {
                if *x >= fold.position {
                    [2 * fold.position - *x, *y]
                } else {
                    [*x, *y]
                }
            } else {
                if *y >= fold.position {
                    [*x, 2 * fold.position - *y]
                } else {
                    [*x, *y]
                }
            }
        })
        .collect()
}

fn parse_fold_string(s: String) -> Fold {
    let mut iter = s.split_whitespace();
    let fold = iter.next().unwrap();
    assert!(fold == "fold");
    let along = iter.next().unwrap();
    assert!(along == "along");
    let mut pos = iter.next().unwrap().split("=");
    let axis = pos.next().unwrap();
    let value = pos.next().unwrap().parse::<i32>().unwrap();
    Fold {
        vertical: match axis {
            "x" => true,
            "y" => false,
            _ => panic!("Unknown axis"),
        },
        position: value,
    }
}

fn main() {
    let mut line_iter = read_lines("input").unwrap().map(|l| (l.unwrap()));
    let mut dots: HashSet<[i32; 2]> = line_iter
        .by_ref()
        .take_while(|s| s.contains(','))
        .map(parse_point)
        .collect();
    let _ = line_iter.by_ref().take_while(|s| s.is_empty());
    let folds: Vec<Fold> = line_iter.map(parse_fold_string).collect();
    dots = fold(dots, &folds[0]);
    println!("{}", dots.len());
    for f in folds.iter().skip(1) {
        dots = fold(dots, f);
    }
    let minx = dots.iter().map(|[x, _]| *x).min().unwrap();
    let maxx = dots.iter().map(|[x, _]| *x).max().unwrap();
    let miny = dots.iter().map(|[_, y]| *y).min().unwrap();
    let maxy = dots.iter().map(|[_, y]| *y).max().unwrap();
    let mut paper = vec![vec![false; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
    for [x, y] in dots {
        paper[(y - miny) as usize][(x - minx) as usize] = true;
    }
    for y in miny..=maxy {
        for x in minx..=maxx {
            print!(
                "{}",
                if paper[(y - miny) as usize][(x - minx) as usize] {
                    '#'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
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
