use std::collections::{HashMap, HashSet};

struct Node {
    id: String,
    outputs: HashSet<String>,
}

#[derive(Eq, PartialEq, Hash)]
struct CacheKey {
    id: String,
    needs_fft: bool,
    needs_dac: bool,
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

fn follow_path(
    machines: &HashMap<String, Node>,
    mut target: CacheKey,
    cache: &mut HashMap<CacheKey, i64>,
) -> i64 {
    match target.id.as_str() {
        "out" => {
            return if target.needs_dac || target.needs_fft {
                0
            } else {
                1
            };
        }
        "dac" => {
            target.needs_dac = false;
        }
        "fft" => {
            target.needs_fft = false;
        }
        _ => {}
    }

    if cache.contains_key(&target) {
        return cache[&target];
    }

    if machines.contains_key(&target.id) {
        let count = machines[&target.id]
            .outputs
            .iter()
            .map(|n| {
                follow_path(
                    machines,
                    CacheKey {
                        id: n.clone(),
                        needs_fft: target.needs_fft,
                        needs_dac: target.needs_dac,
                    },
                    cache,
                )
            })
            .sum();
        cache.insert(target, count);
        count
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

    let you = CacheKey {
        id: String::from("you"),
        needs_dac: false,
        needs_fft: false,
    };
    let paths = follow_path(&machines, you, &mut HashMap::new());

    println!("Found Paths: {}", paths);

    let svr = CacheKey {
        id: String::from("svr"),
        needs_dac: true,
        needs_fft: true,
    };
    let paths_pt2 = follow_path(&machines, svr, &mut HashMap::new());
    println!("Found Paths: {}", paths_pt2);

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}
