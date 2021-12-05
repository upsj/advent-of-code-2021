use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines: Vec<String> = read_lines("input").unwrap().map(|s| s.unwrap()).collect();
    let mut iter = lines.into_iter().peekable();
    let numbers: Vec<i32> = iter
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut fields = Vec::new();
    while iter.peek().is_some() {
        iter.next();
        fields.push(read_bingo_field(iter.by_ref().take(5).collect()))
    }
    let num_fields = fields.len();
    let mut number_it = numbers.iter();
    let mut number_won = 0;
    while number_won == 0 {
        let number = *number_it.next().unwrap();
        for mut field in &mut fields {
            mark_number(&mut field, number);
            if field.won {
                println!("First Score: {}", calculate_score(field, number));
                number_won += 1;
                break;
            }
        }
    }
    while number_won < num_fields {
        let number = *number_it.next().unwrap();
        for mut field in &mut fields {
            if !field.won {
                mark_number(&mut field, number);
                if field.won {
                    number_won += 1
                }
                if number_won == num_fields {
                    println!("Last Score: {}", calculate_score(field, number));
                }
            }
        }
    }
}

struct BingoField {
    data: Vec<Vec<(i32, bool)>>,
    row_won: Vec<bool>,
    col_won: Vec<bool>,
    won: bool,
}

impl fmt::Display for BingoField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", if self.won { "*" } else { " " })?;
        for col in 0..5 {
            write!(f, "  {:2}", if self.col_won[col] { "*" } else { " " })?;
        }
        write!(f, "\n")?;
        for row in 0..5 {
            write!(f, "{} ", if self.row_won[row] { "*" } else { " " })?;
            for col in 0..5 {
                let entry = self.data[row][col];
                write!(f, "{}{:<4}", if entry.1 { "*" } else { " " }, entry.0)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_bingo_field(lines: Vec<String>) -> BingoField {
    BingoField {
        data: lines
            .iter()
            .map(|s| {
                s.split_whitespace()
                    .map(|s| (s.parse::<i32>().unwrap(), false))
                    .collect()
            })
            .collect(),
        row_won: vec![false; 5],
        col_won: vec![false; 5],
        won: false,
    }
}

fn calculate_score(field: &BingoField, number: i32) -> i32 {
    let sum_unmarked = field
        .data
        .iter()
        .map(|r| {
            r.iter()
                .map(|(e, w)| match w {
                    true => 0,
                    false => *e,
                })
                .fold(0, |a, b| a + b)
        })
        .fold(0, |a, b| a + b);
    sum_unmarked * number
}

fn mark_number(field: &mut BingoField, number: i32) {
    for row in field.data.iter_mut() {
        for entry in row.iter_mut() {
            if entry.0 == number {
                entry.1 = true;
            }
        }
    }
    for i in 0..5 {
        field.row_won[i] = (0..5).map(|j| field.data[i][j].1).fold(true, |a, b| a && b);
        field.col_won[i] = (0..5).map(|j| field.data[j][i].1).fold(true, |a, b| a && b);
        field.won = field.won || field.row_won[i] || field.col_won[i];
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
