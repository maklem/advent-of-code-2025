use std::cmp::{max, min};

struct FreshIngredients {
    start: i64,
    end: i64,
}

impl FreshIngredients {
    pub fn new(start: i64, end: i64) -> Self {
        FreshIngredients { start, end }
    }

    pub fn contains(&self, ingredient_id: i64) -> bool {
        self.start <= ingredient_id && ingredient_id <= self.end
    }

    pub fn intersects(&self, other: &Self) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
            || (other.start <= self.start && self.start <= other.end)
            || (other.start <= self.start && self.start <= other.end)
    }

    pub fn merge(&mut self, other: &Self) -> () {
        self.start = min(self.start, other.start);
        self.end = max(self.end, other.end);
    }
}

enum ReaderStage {
    ReadingFreshRanges,
    ReadingIngredients,
}

fn main() {
    let start = std::time::Instant::now();

    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let mut fresh_storage: Vec<FreshIngredients> = vec![];
    let mut ingredients: Vec<i64> = vec![];

    let mut reader_stage = ReaderStage::ReadingFreshRanges;
    for line in code.lines() {
        match reader_stage {
            ReaderStage::ReadingFreshRanges => {
                if line.is_empty() {
                    reader_stage = ReaderStage::ReadingIngredients;
                    continue;
                }
                let parts: Vec<&str> = line.split("-").collect();
                let start = parts[0].parse().unwrap();
                let end = parts[1].parse().unwrap();
                fresh_storage.push(FreshIngredients::new(start, end));
            }
            ReaderStage::ReadingIngredients => {
                ingredients.push(line.parse().unwrap());
            }
        }
    }

    // Part 2
    // Step 1: Merge Ranges of fresh storage.
    // Step 1.1: Sort by x.start
    //   any later entry will only ever intersect with the last one
    //   for being able to merge ranges in one pass.
    fresh_storage.sort_by(|a, b| {
        let diff = a.start - b.start;
        if diff > 0 {
            std::cmp::Ordering::Greater
        } else if diff < 0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    // Step 1.2: merge overlapping ranges
    let mut merged_fresh_storage: Vec<FreshIngredients> = vec![];
    for entry in &fresh_storage {
        let mut merged = false;
        if let Some(storage) = merged_fresh_storage.last_mut() {
            if storage.intersects(entry) {
                storage.merge(entry);
                merged = true;
            }
        }
        if !merged {
            merged_fresh_storage.push(FreshIngredients {
                start: entry.start,
                end: entry.end,
            });
        }
    }

    // Part 1: Count fresh ingredients
    let mut fresh_count = 0;
    for ingredient in ingredients {
        for fresh_storage in &merged_fresh_storage {
            if fresh_storage.contains(ingredient) {
                fresh_count += 1;
                break;
            }
        }
    }
    println!("[Part 1] fresh ingredients found: {}", fresh_count);

    // Part 2
    // Step 2: sum width of fresh ingredient ranges
    fresh_count = 0;
    for fresh_storage in &merged_fresh_storage {
        fresh_count += fresh_storage.end - fresh_storage.start + 1;
    }
    println!("[Part 2] fresh ingredients found: {}", fresh_count);

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}
