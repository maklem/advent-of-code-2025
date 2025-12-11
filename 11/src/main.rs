use std::collections::{HashMap, HashSet};

struct Node {
    id: String,
    outputs: HashSet<String>,
}

impl Node {
    fn from_str(line: &str) -> Self {
        let mut iter = line.split(" ");
        let id = iter.next().unwrap();
        let outputs = iter.map(|s| s.to_string()).collect();

        Self {
            id: id[0..3].to_string(),
            outputs,
        }
    }
}

fn follow_path(machines: &HashMap<String, Node>, node: &String, _path_factor: i64) -> i64 {
    if node == "out" {
        return 1;
    }
    if machines.contains_key(node) {
        machines[node]
            .outputs
            .iter()
            .map(|n| follow_path(machines, n, _path_factor))
            .sum()
    } else {
        0
    }
}

fn main() {
    let start = std::time::Instant::now();
    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let machines: HashMap<_, _> = code
        .lines()
        .map(Node::from_str)
        .map(|n| (n.id.clone(), n))
        .collect();

    let you = String::from("you");
    let paths = follow_path(&machines, &you, 1);

    println!("Found Paths: {}", paths);

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}
