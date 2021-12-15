use std::cmp::min_by_key;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct HeapEntry {
    x: usize,
    y: usize,
    dist: usize,
}

#[derive(Clone, Copy)]
struct NodeInfo {
    px: usize,
    py: usize,
    dist: usize,
}

static SENTINEL: HeapEntry = HeapEntry {
    x: usize::MAX,
    y: usize::MAX,
    dist: usize::MAX,
};

fn sift_down(pq: &mut Vec<HeapEntry>, heap_handles: &mut Vec<Vec<Option<usize>>>, mut pos: usize) {
    while (2 * pos + 1) < pq.len() {
        let lchild = &pq[2 * pos + 1];
        let rchild = pq.get(2 * pos + 2).unwrap_or(&SENTINEL);
        let minchild = min_by_key((lchild, 2 * pos + 1), (rchild, 2 * pos + 2), |(a, _)| {
            a.dist
        });
        let cur = &pq[pos];
        if cur.dist <= minchild.0.dist {
            break;
        }
        heap_handles[cur.y][cur.x] = Some(minchild.1);
        heap_handles[minchild.0.y][minchild.0.x] = Some(pos);
        let minpos = minchild.1;
        pq.swap(pos, minpos);
        pos = minpos;
    }
}

fn sift_up(pq: &mut Vec<HeapEntry>, heap_handles: &mut Vec<Vec<Option<usize>>>, mut pos: usize) {
    while pos > 0 {
        let parent_pos = (pos - 1) / 2;
        let cur = &pq[pos];
        let parent = &pq[parent_pos];
        if cur.dist >= parent.dist {
            break;
        }
        heap_handles[cur.y][cur.x] = Some(parent_pos);
        heap_handles[parent.y][parent.x] = Some(pos);
        pq.swap(pos, parent_pos);
        pos = parent_pos;
    }
}

fn pop_min(pq: &mut Vec<HeapEntry>, heap_handles: &mut Vec<Vec<Option<usize>>>) -> HeapEntry {
    let last = pq.len() - 1;
    pq.swap(0, last);
    let min = pq.pop().unwrap();
    if !pq.is_empty() {
        sift_down(pq, heap_handles, 0);
    }
    min
}

fn print_path(nodes: Vec<Vec<NodeInfo>>) {
    let height = nodes.len();
    let width = nodes[0].len();
    let mut part_of_path = vec![vec![false; width]; height];
    part_of_path[0][0] = true;
    let mut x = width - 1;
    let mut y = height - 1;
    while x != 0 || y != 0 {
        part_of_path[y][x] = true;
        let node = &nodes[y][x];
        x = node.px;
        y = node.py;
    }
    for y in 0..height {
        for x in 0..width {
            if part_of_path[y][x] {
                print!("█");
            } else {
                let dx = nodes[y][x].px as i32 - x as i32;
                let dy = nodes[y][x].py as i32 - y as i32;
                match (dx, dy) {
                    (-1, 0) => print!("←"),
                    (1, 0) => print!("→"),
                    (0, -1) => print!("↑"),
                    (0, 1) => print!("↓"),
                    _ => panic!("Unknown direction"),
                }
            }
        }
        println!();
    }
}

fn run_dijkstra(dangers: &Vec<Vec<usize>>) -> Vec<Vec<NodeInfo>> {
    let width = dangers[0].len();
    let height = dangers.len();
    //let mut parents = vec![vec![None; width]; height];
    let mut heap_handles = vec![vec![None; width]; height];
    let mut nodes = vec![
        vec![
            NodeInfo {
                px: usize::MAX,
                py: usize::MAX,
                dist: usize::MAX
            };
            width
        ];
        height
    ];
    let mut pq = vec![HeapEntry {
        x: 0,
        y: 0,
        dist: 0,
    }];
    heap_handles[0][0] = Some(0);
    nodes[0][0] = NodeInfo {
        px: 0,
        py: 0,
        dist: 0,
    };
    while !pq.is_empty() {
        let top = pop_min(&mut pq, &mut heap_handles);
        if top.x == width - 1 && top.y == height - 1 {
            break;
        }
        for (nx, ny) in [(-1 as i32, 0), (1, 0), (0, -1 as i32), (0, 1)]
            .iter()
            .map(|(dx, dy)| (top.x as i32 + dx, top.y as i32 + dy))
            .filter(|(x, y)| x >= &0 && y >= &0 && (*x as usize) < width && (*y as usize) < height)
            .map(|(x, y)| (x as usize, y as usize))
        {
            let cur_node = &nodes[top.y][top.x];
            let target_node = &nodes[ny][nx];
            let current_dist = target_node.dist;
            let new_dist = cur_node.dist + dangers[ny][nx];
            if new_dist < current_dist {
                match heap_handles[ny][nx] {
                    Some(pos) => {
                        pq[pos].dist = new_dist;
                        sift_up(&mut pq, &mut heap_handles, pos);
                    }
                    None => {
                        let pos = pq.len();
                        pq.push(HeapEntry {
                            x: nx,
                            y: ny,
                            dist: new_dist,
                        });
                        heap_handles[ny][nx] = Some(pos);
                        sift_up(&mut pq, &mut &mut heap_handles, pos);
                    }
                };
                nodes[ny][nx] = NodeInfo {
                    px: top.x,
                    py: top.y,
                    dist: new_dist,
                };
            }
        }
    }
    nodes
}

fn main() {
    let dangers: Vec<Vec<usize>> = read_lines("input")
        .unwrap()
        .map(|s| {
            s.unwrap()
                .as_str()
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let nodes = run_dijkstra(&dangers);
    println!("{}", nodes.last().unwrap().last().unwrap().dist);
    // print_path(nodes);
    let tiled_dangers: Vec<Vec<usize>> = (0..5)
        .flat_map(|y| {
            dangers.iter().map(move |row| {
                (0..5)
                    .flat_map(|x| row.iter().map(move |d| (d - 1 + x + y) % 9 + 1))
                    .collect()
            })
        })
        .collect();
    let tiled_nodes = run_dijkstra(&tiled_dangers);
    println!("{}", tiled_nodes.last().unwrap().last().unwrap().dist);
    // print_path(tiled_nodes);
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
