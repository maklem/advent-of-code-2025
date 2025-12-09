use std::cmp::{max, min};

#[derive(Copy, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
}

fn to_coordinates(line: &str) -> Coordinate {
    let values: Vec<_> = line.split(",").map(|x| x.parse::<i64>()).collect();
    Coordinate {
        x: values[0].clone().unwrap(),
        y: values[1].clone().unwrap(),
    }
}

struct Rectangle {
    min: Coordinate,
    max: Coordinate,
}

impl Rectangle {
    fn new(a: Coordinate, b: Coordinate) -> Self {
        Self {
            min: Coordinate {
                x: min(a.x, b.x),
                y: min(a.y, b.y),
            },
            max: Coordinate {
                x: max(a.x, b.x),
                y: max(a.y, b.y),
            },
        }
    }
    fn area(&self) -> i64 {
        let dx = (self.min.x - self.max.x).abs() + 1;
        let dy = (self.min.y - self.max.y).abs() + 1;
        dx * dy
    }

    fn intersects(&self, other: &Self) -> bool {
        !(other.max.x <= self.min.x
            || self.max.x <= other.min.x
            || other.max.y <= self.min.y
            || self.max.y <= other.min.y)
    }
}
fn main() {
    let start = std::time::Instant::now();
    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let coordinates: Vec<_> = code.lines().map(to_coordinates).collect();
    let edges: Vec<_> = (0..coordinates.len())
        .map(|i| {
            let index2 = (i + 1) % coordinates.len();
            Rectangle::new(coordinates[i], coordinates[index2])
        })
        .collect();

    let mut area_max = 0;
    let mut area_max_part2 = 0;
    for (index, coordinate) in coordinates.iter().enumerate() {
        for position in &coordinates[index + 1..] {
            let rect = Rectangle::new(*coordinate, *position);
            let current_area = rect.area();

            if current_area > area_max {
                area_max = current_area;
            }

            if current_area > area_max_part2 {
                let intersects = edges.iter().any(|x| rect.intersects(x));
                if !intersects {
                    area_max_part2 = current_area;
                }
            }
        }
    }

    println!("Maximal area is:     {:>12}", area_max);
    println!("Constrained area is: {:>12}", area_max_part2);

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}
