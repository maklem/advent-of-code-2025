fn part1(lines: &Vec<&str>) -> (i64, i64) {
    let mut total = 0;
    let mut beams: Vec<i64> = vec![];
    for line in lines {
        if beams.is_empty() {
            beams = vec![0; line.len()];
        }

        for (index, entry) in line.chars().enumerate() {
            if entry == 'S' {
                beams[index] = 1;
            }
            if entry == '^' && beams[index] != 0 {
                beams[index - 1] += beams[index];
                beams[index + 1] += beams[index];
                beams[index] = 0;
                total += 1;
            }
        }
    }
    let timelines = beams.iter().sum();
    (total, timelines)
}

fn main() {
    let start = std::time::Instant::now();

    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let lines: Vec<&str> = code.lines().collect();

    let (splits, timelines) = part1(&lines);
    println!("[Part 1] Splits    = {}", splits);
    println!("[Part 2] Timelines = {}", timelines);
    //println!("[Part 2] Total = {}", part2(&lines));

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}
