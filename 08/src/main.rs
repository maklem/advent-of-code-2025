use std::collections::HashSet;

struct Position {
    x: i64,
    y: i64,
    z: i64,
}

struct JunctionBox {
    position: Position,
}

struct JunctionDistance {
    index_a: usize,
    index_b: usize,
    distance_squared: i64,
}

struct JunctionNetwork {
    indices: HashSet<usize>,
}

impl JunctionNetwork {
    pub fn contains(&self, index: usize) -> bool {
        self.indices.contains(&index)
    }

    pub fn add(&mut self, index: usize) {
        if !self.indices.contains(&index) {
            self.indices.insert(index);
        }
    }

    pub fn size(&self) -> usize {
        self.indices.len()
    }

    pub fn merge_from(&mut self, other: &Self) {
        for index in &other.indices {
            if !self.contains(*index) {
                self.add(*index);
            }
        }
    }
}

fn to_junction_box(line: &str) -> JunctionBox {
    let coordinates: Vec<_> = line.split(",").collect();

    JunctionBox {
        position: Position {
            x: coordinates[0].parse().unwrap(),
            y: coordinates[1].parse().unwrap(),
            z: coordinates[2].parse().unwrap(),
        },
    }
}

fn main() {
    let start = std::time::Instant::now();

    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let junction_boxes: Vec<_> = code.lines().map(to_junction_box).collect();

    println!(
        "reading input completed at {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    );

    let mut junction_distances: Vec<JunctionDistance> = vec![];

    for (index_a, junction_a) in junction_boxes.iter().enumerate() {
        for (offset_b, junction_b) in junction_boxes[index_a + 1..].iter().enumerate() {
            let distance_squared = (junction_a.position.x - junction_b.position.x)
                * (junction_a.position.x - junction_b.position.x)
                + (junction_a.position.y - junction_b.position.y)
                    * (junction_a.position.y - junction_b.position.y)
                + (junction_a.position.z - junction_b.position.z)
                    * (junction_a.position.z - junction_b.position.z);
            if distance_squared <= 200000000 {
                junction_distances.push(JunctionDistance {
                    index_a,
                    index_b: index_a + offset_b + 1,
                    distance_squared,
                });
            }
        }
    }

    println!(
        "distances computed at {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    );
    junction_distances.sort_by_key(|e| e.distance_squared);

    println!(
        "distances sorted at {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    );

    let mut part1_grand_product = 0;
    let mut part2_final_connection = 0;
    let mut part2_final_distance = 0;

    let mut junction_networks: Vec<JunctionNetwork> = vec![];
    for (connection_id, connection) in junction_distances.iter().enumerate() {
        if connection_id == 1000 {
            junction_networks.sort_by_key(|e| usize::MAX - e.indices.len());
            part1_grand_product = junction_networks[0].size()
                * junction_networks[1].size()
                * junction_networks[2].size();
        }

        let mut primary_network = Option::<usize>::None;

        for network_id in 0..junction_networks.len() {
            if junction_networks[network_id].contains(connection.index_a)
                || junction_networks[network_id].contains(connection.index_b)
            {
                if let Some(primary_id) = primary_network {
                    let merged_network = junction_networks.remove(network_id);
                    junction_networks
                        .get_mut(primary_id)
                        .unwrap()
                        .merge_from(&merged_network);
                    break; // there can only be one other network
                } else {
                    junction_networks[network_id].add(connection.index_a);
                    junction_networks[network_id].add(connection.index_b);
                    primary_network = Some(network_id);
                }
            }
        }

        if primary_network.is_none() {
            let mut indices = HashSet::new();
            indices.insert(connection.index_a);
            indices.insert(connection.index_b);
            junction_networks.push(JunctionNetwork { indices });
        }

        if !junction_networks.is_empty()
            && junction_networks.first().unwrap().size() == junction_boxes.len()
        {
            part2_final_connection = junction_boxes[connection.index_a].position.x
                * junction_boxes[connection.index_b].position.x;
            part2_final_distance = connection.distance_squared;
            break;
        }
    }

    println!("[Part 1] Grand Product    = {}", part1_grand_product);
    println!("[Part 2] Final Connection = {}", part2_final_connection);
    println!("[Part 2] Final Distance^2 = {}", part2_final_distance);

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}
