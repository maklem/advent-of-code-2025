fn convert_to_numbers(line: &str) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    for entry in line.split(' ') {
        if let Ok(number) = entry.parse::<i64>() {
            result.push(number);
        }
    }
    result
}

#[derive(Debug, PartialEq)]
enum MathOperation {
    Add,
    Multiply,
}

fn convert_to_operation(line: &str) -> Vec<MathOperation> {
    let mut result: Vec<MathOperation> = vec![];
    for entry in line.split(' ') {
        match entry {
            "" => {}
            "+" => result.push(MathOperation::Add),
            "*" => result.push(MathOperation::Multiply),
            _ => panic!("Unknown math operation >{}<", entry),
        };
    }
    result
}

fn part1(lines: &[&str]) -> i64 {
    let line0 = convert_to_numbers(lines[0]);
    let line1 = convert_to_numbers(lines[1]);
    let line2 = convert_to_numbers(lines[2]);
    let line3 = convert_to_numbers(lines[3]);
    let operations = convert_to_operation(lines[4]);

    let mut total = 0;
    for index in 0..operations.len() {
        total += match operations[index] {
            MathOperation::Add => line0[index] + line1[index] + line2[index] + line3[index],
            MathOperation::Multiply => line0[index] * line1[index] * line2[index] * line3[index],
        }
    }
    total
}

fn parse_vertical_number(lines: &Vec<&str>, index: usize) -> Option<i64> {
    let mut vertical_number = String::new();
    for line in lines.iter().take(4) {
        let symbol = line.chars().nth(index).unwrap().to_string();
        vertical_number += match symbol.as_str() {
            " " => "",
            _ => &symbol,
        }
    }
    if vertical_number.is_empty() {
        None
    } else {
        Some(vertical_number.parse::<i64>().unwrap())
    }
}

fn parse_math_operation(line: &str, index: usize) -> Option<MathOperation> {
    let symbol = line.chars().nth(index).unwrap().to_string();
    match symbol.as_str() {
        "+" => Some(MathOperation::Add),
        "*" => Some(MathOperation::Multiply),
        " " => None,
        _ => panic!("could not parse math operation >{}<", symbol),
    }
}

fn part2(lines: &Vec<&str>) -> i64 {
    let mut total = 0;

    let mut current_operation = MathOperation::Add;
    let mut current_value = 0;
    for index in 0..lines[0].len() {
        if let Some(operation) = parse_math_operation(lines[4], index) {
            current_operation = operation;
            total += current_value;
            current_value = match &current_operation {
                MathOperation::Add => 0,
                MathOperation::Multiply => 1,
            };
        }

        if let Some(number) = parse_vertical_number(lines, index) {
            current_value = match &current_operation {
                MathOperation::Add => current_value + number,
                MathOperation::Multiply => current_value * number,
            };
        }
    }

    total + current_value
}

fn main() {
    let start = std::time::Instant::now();

    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let lines: Vec<&str> = code.lines().collect();

    println!("[Part 1] Total = {}", part1(&lines));
    println!("[Part 2] Total = {}", part2(&lines));

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::{MathOperation, convert_to_numbers, convert_to_operation, parse_vertical_number};

    #[test]
    fn convert_to_numbers__converts_string_to_numbers() {
        let input = "1 2     3 4";
        let result = convert_to_numbers(input);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn convert_to_operation__converts_math_operations() {
        let input = "+ *    + *";
        let result = convert_to_operation(input);
        assert_eq!(
            result,
            vec![
                MathOperation::Add,
                MathOperation::Multiply,
                MathOperation::Add,
                MathOperation::Multiply,
            ]
        );
    }

    #[test]
    fn parse_vertical_number__evaluates_number() {
        let input = vec!["11  ", " 22 ", "  3 ", "  0 "];
        assert_eq!(Some(1), parse_vertical_number(&input, 0));
        assert_eq!(Some(12), parse_vertical_number(&input, 1));
        assert_eq!(Some(230), parse_vertical_number(&input, 2));
        assert_eq!(None, parse_vertical_number(&input, 3));
    }
}
