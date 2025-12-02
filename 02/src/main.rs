use std::mem::replace;

enum ReaderState {
    ReadingStart { buffer: String },
    ReadingEnd { start: String, buffer: String },
    DoneReading,
    InvalidState,
}

struct Reader {
    state: ReaderState,
}

struct ProductRange {
    start: i64,
    end: i64,
}

fn read_start(input: char, mut buffer: String) -> ReaderState {
    match input {
        '0'..='9' => {
            buffer.push(input);
            ReaderState::ReadingStart { buffer }
        }
        '-' => ReaderState::ReadingEnd {
            start: buffer,
            buffer: String::new(),
        },
        _ => panic!("unexpected character {} in read_start", input),
    }
}

fn read_end(
    input: char,
    start: String,
    mut buffer: String,
    result: &mut Option<ProductRange>,
) -> ReaderState {
    match input {
        '0'..='9' => {
            buffer.push(input);
            ReaderState::ReadingEnd { start, buffer }
        }
        ',' => {
            let num_start = start.parse::<i64>().unwrap();
            let num_end = buffer.parse::<i64>().unwrap();
            *result = Some(ProductRange {
                start: num_start,
                end: num_end,
            });
            ReaderState::ReadingStart {
                buffer: String::new(),
            }
        }
        '\r' => ReaderState::ReadingEnd { start, buffer },
        '\n' => {
            let num_start = start.parse::<i64>().unwrap();
            let num_end = buffer.parse::<i64>().unwrap();
            *result = Some(ProductRange {
                start: num_start,
                end: num_end,
            });
            ReaderState::DoneReading
        }
        _ => panic!("unexpected character {} in read_end", input),
    }
}

impl Reader {
    pub fn new() -> Self {
        Reader {
            state: ReaderState::ReadingStart {
                buffer: String::new(),
            },
        }
    }

    pub fn read(&mut self, input: char) -> Option<ProductRange> {
        let state = replace(&mut self.state, ReaderState::InvalidState);
        let mut product_range = Option::<ProductRange>::None;

        self.state = match state {
            ReaderState::ReadingStart { buffer } => read_start(input, buffer),
            ReaderState::ReadingEnd { start, buffer } => {
                read_end(input, start, buffer, &mut product_range)
            }
            ReaderState::DoneReading => panic!("received data after end"),
            ReaderState::InvalidState => panic!("received data in temporary invalid state"),
        };

        product_range
    }
}

fn number_length(num: i64) -> i64 {
    (num as f64).log10() as i64 + 1
}

fn pow10(exponent: i64) -> i64 {
    let mut num = 1i64;
    for _ in 1..=exponent {
        num *= 10
    }
    num
}

fn is_valid_product_part1(id: i64) -> bool {
    let length = number_length(id);
    if length % 2 != 0 {
        return true;
    }
    let factor = pow10(length / 2) + 1;

    id % factor != 0
}

fn find_invalid_products_part1(products: &ProductRange) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];

    for id in products.start..=products.end {
        if !is_valid_product_part1(id) {
            result.push(id);
        }
    }

    result
}

fn is_valid_product_part2(id: i64) -> bool {
    let length = number_length(id);

    for segment_length in 1..10 {
        if length % segment_length != 0 {
            continue;
        }
        let repetitions = length / segment_length;
        if repetitions <= 1 {
            continue;
        }

        let mut factor = 1;
        for _ in 1..repetitions {
            factor *= pow10(segment_length);
            factor += 1;
        }

        if id % factor == 0 {
            return false;
        }
    }
    true
}

fn find_invalid_products_part2(products: &ProductRange) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];

    for id in products.start..=products.end {
        if !is_valid_product_part2(id) {
            result.push(id);
        }
    }

    result
}

fn main() {
    let mut reader = Reader::new();
    let mut invalid_products_part1: Vec<i64> = vec![];
    let mut invalid_products_part2: Vec<i64> = vec![];

    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };

    for input in code.chars() {
        let event = reader.read(input);
        if let Some(products) = event {
            let new_invalid_products_part1 = find_invalid_products_part1(&products);
            for id in new_invalid_products_part1 {
                invalid_products_part1.push(id);
            }
            let new_invalid_products_part2 = find_invalid_products_part2(&products);
            for id in new_invalid_products_part2 {
                invalid_products_part2.push(id);
            }
        }
    }
    print!("== Part 1 ==\n\n");
    println!(
        "Invalid product ids found: {}",
        invalid_products_part1.len()
    );

    let mut summarized = 0;
    for id in invalid_products_part1 {
        summarized += id;
    }
    println!("sum of invalid product ids: {}", summarized);

    print!("\n\n== Part 2 ==\n\n");
    println!(
        "Invalid product ids found: {}",
        invalid_products_part2.len()
    );
    summarized = 0;
    for id in invalid_products_part2 {
        summarized += id;
    }
    println!("sum of invalid product ids: {}", summarized);
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::{
        ProductRange, Reader, find_invalid_products_part1, is_valid_product_part1, read_end,
    };

    #[test]
    fn is_invalid_product_id__detects_invalid_products() {
        assert!(!is_valid_product_part1(22));
        assert!(!is_valid_product_part1(2020));
        assert!(!is_valid_product_part1(123123));
    }

    #[test]
    fn find_invalid_products_part1__when_called__finds_expected_invalid_ids() {
        assert!(
            find_invalid_products_part1(&crate::ProductRange { start: 10, end: 25 }).len() == 2
        );
        assert!(
            find_invalid_products_part1(&crate::ProductRange {
                start: 1000,
                end: 1234
            })
            .len()
                == 3
        );
    }

    #[test]
    fn read_end__on_comma__returns_ProductRange() {
        let start = "1009".to_string();
        let end = "1011".to_string();
        let mut result = Option::<ProductRange>::None;

        let _state = read_end(',', start, end, &mut result);

        assert!(result.is_some());
    }

    #[test]
    fn read_end__on_linebreak__returns_ProductRange() {
        let start = "1009".to_string();
        let end = "1011".to_string();
        let mut result = Option::<ProductRange>::None;

        let _state = read_end('\n', start, end, &mut result);

        assert!(result.is_some());
    }

    #[test]
    fn Reader__given_a_string__returns_ProductRange() {
        let inputstream = "4000-4045\n".to_string();

        let mut reader = Reader::new();
        let mut count = 0;

        for input in inputstream.chars() {
            let result = reader.read(input);
            if let Some(ids) = result {
                count += 1;

                assert!(ids.start == 4000);
                assert!(ids.end == 4045);
            }
        }

        assert!(count == 1);
    }
}
