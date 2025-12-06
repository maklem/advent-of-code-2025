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

fn part1(lines: &Vec<&str>) -> i64 {
    let line0 = convert_to_numbers(lines[0]);
    let line1 = convert_to_numbers(lines[1]);
    let line2 = convert_to_numbers(lines[2]);
    let line3 = convert_to_numbers(lines[3]);
    let operations = convert_to_operation(lines[4]);

    let mut total = 0;
    for index in 0..operations.len() {
        total += match operations[index] {
            MathOperation::Add => {
                line0[index] + line1[index] + line2[index] + line3[index]
            },
            MathOperation::Multiply => {
                line0[index] * line1[index] * line2[index] * line3[index]
            },            
        }
    }
    total
}


fn main() {
    let start = std::time::Instant::now();

    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };

    let lines: Vec<&str> = code.lines().collect();
    let total = part1(&lines);

    println!("[Part 1] Total = {}", total);

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::{MathOperation, convert_to_numbers, convert_to_operation};

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
}
