use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut grid: Vec<Vec<_>> = read_lines("input")
        .unwrap()
        .map(|s| {
            s.unwrap()
                .as_str()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();
    let mut num_flashes = 0;
    let mut step = 0;
    loop {
        let mut flashes: Vec<(i32, i32)> = Vec::new();
        let mut flash_ptr: usize = 0;
        for y in 0..10 {
            for x in 0..10 {
                let v = &mut grid[y as usize][x as usize];
                *v += 1;
                if *v > 9 {
                    flashes.push((x, y));
                }
            }
        }
        while flash_ptr < flashes.len() {
            let (x, y) = flashes[flash_ptr];
            num_flashes += 1;
            flash_ptr += 1;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if (0..10).contains(&nx) && (0..10).contains(&ny) {
                        let v = &mut grid[ny as usize][nx as usize];
                        match v {
                            0..=8 => *v += 1,
                            9 => {
                                *v = 10;
                                flashes.push((nx, ny));
                            }
                            _ => {}
                        };
                    }
                }
            }
        }
        step += 1;
        if step == 100 {
            println!("num flashes: {}", num_flashes);
        }
        if flashes.len() == 100 {
            println!("synchronized at: {}", step);
            break;
        }
        for (x, y) in flashes {
            grid[y as usize][x as usize] = 0;
        }
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
