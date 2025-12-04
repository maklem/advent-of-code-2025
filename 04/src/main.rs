struct PaperRollStorage {
    storage: Vec<Vec<bool>>,
    rows: i32,
    cols: i32,
}

impl PaperRollStorage {
    pub fn new(input: String) -> Self {
        let lines = input.lines();
        let mut rows = 0;
        let mut cols = 0;
        let mut storage = Vec::<Vec<bool>>::new();

        for (index, line) in lines.enumerate() {
            if cols == 0 {
                cols = line.len();
            }
            if cols != line.len() {
                panic!("non grid shape")
            }
            let mut linestorage = Vec::<bool>::new();
            for entry in line.as_bytes() {
                linestorage.push(match entry {
                    b'.' => false,
                    b'@' => true,
                    _ => panic!("non storage entry in grid"),
                });
            }
            storage.push(linestorage);
            rows = index + 1
        }
        PaperRollStorage {
            storage,
            rows: rows as i32,
            cols: cols as i32,
        }
    }

    pub fn take_paper_roll(&mut self, row: i32, col: i32) {
        self.storage[row as usize][col as usize] = false;
    }

    pub fn is_paper_roll(&self, row: i32, col: i32) -> bool {
        if (row < 0) || (col < 0) || (row >= self.rows) || (col >= self.cols) {
            return false;
        }
        self.storage[row as usize][col as usize]
    }

    pub fn count_neighbours(&self, row: i32, col: i32) -> i32 {
        let mut count = 0;
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                if r == row && c == col {
                    continue;
                }
                if self.is_paper_roll(r, c) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let start = std::time::Instant::now();

    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let mut storage = PaperRollStorage::new(code);

    let mut available_paper_rolls = 0;
    for row in 0..storage.rows {
        for col in 0..storage.cols {
            if !storage.is_paper_roll(row, col) {
                continue;
            }

            if storage.count_neighbours(row, col) < 4 {
                available_paper_rolls += 1;
            }
        }
    }
    println!("[Part 1] paper rolls available: {}", available_paper_rolls);

    available_paper_rolls = 0;
    loop {
        let mut taken_this_time = 0;
        for row in 0..storage.rows {
            for col in 0..storage.cols {
                if !storage.is_paper_roll(row, col) {
                    continue;
                }

                if storage.count_neighbours(row, col) < 4 {
                    storage.take_paper_roll(row, col);
                    taken_this_time += 1;
                }
            }
        }

        available_paper_rolls += taken_this_time;
        if taken_this_time == 0 {
            break;
        }
    }
    println!("[Part 2] paper rolls available: {}", available_paper_rolls);

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}
