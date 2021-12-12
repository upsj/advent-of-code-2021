use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Node {
    name: String,
    big: bool,
    endpoint: bool,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
}

fn backtrack_traverse(start: usize, end: usize, graph: &Graph) {
    let n = graph.nodes.len();
    let mut local_visited = vec![false; n];
    local_visited[start] = true;
    backtrack_traverse_internal(local_visited, start, end, graph, String::new());
}

fn backtrack_traverse_small_twice(start: usize, end: usize, graph: &Graph) {
    let n = graph.nodes.len();
    let mut local_visited = vec![false; n];
    local_visited[start] = true;
    backtrack_traverse_internal_small_twice(local_visited, start, end, graph, false, String::new());
}

fn backtrack_traverse_internal(
    visited: Vec<bool>,
    cur: usize,
    end: usize,
    graph: &Graph,
    path: String,
) {
    let new_path = if path.is_empty() {
        graph.nodes[cur].name.to_owned()
    } else {
        path + "," + graph.nodes[cur].name.as_str()
    };
    if cur == end {
        println!("{}", new_path);
    }
    for neighbor in &graph.edges[cur] {
        if !visited[*neighbor] || graph.nodes[*neighbor].big {
            let mut new_visited = visited.to_owned();
            new_visited[*neighbor] = true;
            backtrack_traverse_internal(new_visited, *neighbor, end, graph, new_path.to_owned());
        }
    }
}

fn backtrack_traverse_internal_small_twice(
    visited: Vec<bool>,
    cur: usize,
    end: usize,
    graph: &Graph,
    visited_small_twice: bool,
    path: String,
) {
    let new_path = if path.is_empty() {
        graph.nodes[cur].name.to_owned()
    } else {
        path + "," + graph.nodes[cur].name.as_str()
    };
    if cur == end {
        println!("{}", new_path);
    }
    for neighbor in &graph.edges[cur] {
        let neighbor_node = &graph.nodes[*neighbor];
        let neighbor_visited = visited[*neighbor];
        if !neighbor_visited
            || neighbor_node.big
            || !(neighbor_node.endpoint || visited_small_twice)
        {
            let new_visited_small_twice =
                visited_small_twice || (!graph.nodes[*neighbor].big && visited[*neighbor]);
            let mut new_visited = visited.to_owned();
            new_visited[*neighbor] = true;
            backtrack_traverse_internal_small_twice(
                new_visited,
                *neighbor,
                end,
                graph,
                new_visited_small_twice,
                new_path.to_owned(),
            );
        }
    }
}

fn main() {
    let str_edges: Vec<[String; 2]> = read_lines("input")
        .unwrap()
        .map(|l| {
            (l.unwrap().split('-'))
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let nodes: HashSet<String> = str_edges
        .iter()
        .map(|p| p[0].to_owned())
        .chain(str_edges.iter().map(|p| p[1].to_owned()))
        .collect();
    let mut node_vec: Vec<Node> = nodes
        .into_iter()
        .map(|s| {
            let is_upper = s.as_str().chars().all(|c| c.is_uppercase());
            let is_endpoint = match s.as_str() {
                "start" | "end" => true,
                _ => false,
            };
            Node {
                name: s,
                big: is_upper,
                endpoint: is_endpoint,
            }
        })
        .collect();
    node_vec.sort_by_key(|n| n.name.to_owned());
    let node_map: HashMap<String, usize> = node_vec
        .iter()
        .enumerate()
        .map(|(i, node)| (node.name.to_owned(), i))
        .collect();
    let n = node_map.len();
    let mut edge_vec = vec![Vec::new(); n];
    for edge in str_edges {
        let u = node_map[&edge[0]];
        let v = node_map[&edge[1]];
        edge_vec[u].push(v);
        edge_vec[v].push(u);
    }
    for neighbors in &mut edge_vec {
        neighbors.sort();
    }
    let graph = Graph {
        nodes: node_vec,
        edges: edge_vec,
    };
    backtrack_traverse_small_twice(node_map["start"], node_map["end"], &graph)
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
