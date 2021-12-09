use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let data: Vec<Vec<u8>> = read_lines("input")
        .unwrap()
        .map(|l| {
            l.unwrap()
                .as_str()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();
    let width = data[0].len() as i32;
    let height = data.len() as i32;
    let mut basin_source = vec![vec![-1; width as usize]; height as usize];
    let mut basins = Vec::new();
    let mut sum: u32 = 0;
    let read = |x: i32, y: i32| {
        if x >= 0 && x < width && y >= 0 && y < height {
            data[y as usize][x as usize]
        } else {
            255
        }
    };
    let get_neighbors = |x, y| [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];
    let mut fill_stack = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let center = read(x, y);
            let neighbor_positions = get_neighbors(x, y);
            let neighbors = neighbor_positions.map(|(x, y)| read(x, y));
            let neighbor_min = neighbors.iter().min().unwrap();
            if center < *neighbor_min {
                let mut basin_size = 1;
                sum += 1 + center as u32;
                for ((nx, ny), nh) in neighbor_positions.iter().zip(neighbors.iter()) {
                    if *nh < 9 {
                        fill_stack.push((*nx, *ny));
                        basin_source[*ny as usize][*nx as usize] = basins.len() as i32;
                        basin_size += 1;
                    }
                }
                basin_source[y as usize][x as usize] = basins.len() as i32;
                basins.push((x, y, basin_size));
            }
        }
    }
    while !fill_stack.is_empty() {
        let (x, y) = fill_stack.pop().unwrap();
        let neighbor_positions = get_neighbors(x, y);
        let basin = basin_source[y as usize][x as usize];
        for (nx, ny) in neighbor_positions {
            let nv = read(nx, ny);
            if nv < 9 && basin_source[ny as usize][nx as usize] < 0 {
                fill_stack.push((nx, ny));
                basin_source[ny as usize][nx as usize] = basin;
                basins[basin as usize].2 += 1;
            }
        }
    }
    println!("{}", sum);
    basins.sort_by_key(|v| -v.2);
    println!(
        "{}",
        basins.iter().take(3).map(|v| v.2).fold(1, |a, b| a * b)
    );
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
