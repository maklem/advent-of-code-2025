struct Extremal {
    value: i64,
    position: usize,
}

fn to_numerical_value(symbol: char) -> i64 {
    match &symbol {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("non numerical symbol >{}<", symbol),
    }
}

fn find_largest_symbol(line: &str) -> Extremal {
    let mut current_max = 0;
    let mut current_pos = 0;
    for (position, symbol) in line.chars().enumerate() {
        let value = to_numerical_value(symbol);

        if value > current_max {
            current_max = value;
            current_pos = position;
        }
    }
    Extremal {
        value: current_max,
        position: current_pos,
    }
}

fn find_maximal_power(line: &str, digits: usize) -> i64 {
    let length = line.len();
    let mut current_offset = 0;
    let mut current_value: i64 = 0;

    for position in 0..digits {
        let final_symbol_index = length - digits + position;
        let maximal = find_largest_symbol(&line[current_offset..=final_symbol_index]);
        current_value = current_value * 10 + maximal.value;
        current_offset += maximal.position + 1;
    }

    current_value
}

fn main() {
    let start = std::time::Instant::now();

    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let mut maximal_power_pt1 = 0;
    let mut maximal_power_pt2 = 0;

    for line in code.lines() {
        let power_pt1 = find_maximal_power(line, 2);
        maximal_power_pt1 += power_pt1;

        let power_pt2 = find_maximal_power(line, 12);
        maximal_power_pt2 += power_pt2;
    }

    println!("Part 1 - Summed up power is {}", maximal_power_pt1);
    println!("Part 2 - Summed up power is {}", maximal_power_pt2);
    println!("evaluation took {} ms", start.elapsed().as_nanos() as f64 / 1e6)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::find_maximal_power;

    #[test]
    fn find_maximal_power__with_multiple_maxima__finds_power() {
        let input = "000900800900";
        assert_eq!(99, find_maximal_power(input, 2))
    }

    #[test]
    fn find_maximal_power__with_maximum_at_end__finds_power() {
        let input = "0007008009";
        assert_eq!(89, find_maximal_power(input, 2))
    }
}
