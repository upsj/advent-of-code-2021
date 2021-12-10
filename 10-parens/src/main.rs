use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
enum TokenType {
    Paren,
    Bracket,
    Brace,
    AngleBracket,
}

struct Token {
    opening: bool,
    paren: TokenType,
}

fn parse_char(c: char) -> Token {
    match c {
        '(' => Token {
            opening: true,
            paren: TokenType::Paren,
        },
        '[' => Token {
            opening: true,
            paren: TokenType::Bracket,
        },
        '{' => Token {
            opening: true,
            paren: TokenType::Brace,
        },
        '<' => Token {
            opening: true,
            paren: TokenType::AngleBracket,
        },
        '>' => Token {
            opening: false,
            paren: TokenType::AngleBracket,
        },
        '}' => Token {
            opening: false,
            paren: TokenType::Brace,
        },
        ']' => Token {
            opening: false,
            paren: TokenType::Bracket,
        },
        ')' => Token {
            opening: false,
            paren: TokenType::Paren,
        },
        _ => panic!("Unknown token"),
    }
}

fn token_err_score(t: Token) -> usize {
    assert!(!t.opening);
    match t.paren {
        TokenType::Paren => 3,
        TokenType::Bracket => 57,
        TokenType::Brace => 1197,
        TokenType::AngleBracket => 25137,
    }
}

fn token_close_score(t: &Token) -> usize {
    assert!(t.opening);
    match t.paren {
        TokenType::Paren => 1,
        TokenType::Bracket => 2,
        TokenType::Brace => 3,
        TokenType::AngleBracket => 4,
    }
}

fn parse_line1(l: &String) -> usize {
    let mut stack = Vec::new();
    for c in l.as_str().chars() {
        let tok = parse_char(c);
        if tok.opening {
            stack.push(tok);
        } else {
            let top = stack.pop().unwrap();
            if tok.paren != top.paren {
                return token_err_score(tok);
            }
        }
    }
    0
}

fn parse_line2(l: &String) -> usize {
    let mut stack = Vec::new();
    for c in l.as_str().chars() {
        let tok = parse_char(c);
        if tok.opening {
            stack.push(tok);
        } else {
            let top = stack.pop().unwrap();
            assert!(tok.paren == top.paren);
        }
    }
    stack
        .iter()
        .rev()
        .fold(0, |old, new| 5 * old + token_close_score(new))
}

fn main() {
    let lines: Vec<String> = read_lines("input").unwrap().map(|s| s.unwrap()).collect();
    println!("{}", lines.iter().map(parse_line1).sum::<usize>());
    let mut complete_scores: Vec<usize> = lines
        .iter()
        .filter(|l| parse_line1(l) == 0)
        .map(parse_line2)
        .collect();
    complete_scores.sort();
    println!("{}", complete_scores[complete_scores.len() / 2]);
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
