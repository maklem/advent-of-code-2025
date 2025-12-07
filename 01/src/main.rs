use std::fs;

const INPUT_FILENAME: &str = "input.txt";
const CODE_START: i32 = 50;

#[derive(PartialEq)]
enum CodeReaderState {
    SearchingDirection,
    SearchingCount { positive: bool, buffer: String },
}

struct Move {
    moving_positive: bool,
    length: i32,
}
struct CodeReader {
    state: CodeReaderState,
    moves: Vec<Move>,
}

fn process_direction(input: char) -> CodeReaderState {
    match input {
        'L' => CodeReaderState::SearchingCount {
            positive: false,
            buffer: String::new(),
        },
        'R' => CodeReaderState::SearchingCount {
            positive: true,
            buffer: String::new(),
        },
        _ => {
            panic!("unexpected character in process_direction")
        }
    }
}

fn process_count(
    input: char,
    positive: &bool,
    buffer: &String,
    moves: &mut Vec<Move>,
) -> CodeReaderState {
    match input {
        '0'..='9' => {
            let mut new_buffer = buffer.clone();
            new_buffer.push(input);
            CodeReaderState::SearchingCount {
                positive: *positive,
                buffer: new_buffer,
            }
        }
        '\n' => {
            let length = buffer.parse::<i32>();
            if length.is_err() {
                panic!("could not parse int {} error {:?}", buffer, length.err())
            }
            moves.push(Move {
                moving_positive: *positive,
                length: length.unwrap(),
            });
            CodeReaderState::SearchingDirection
        }
        _ => {
            panic!("unexpected character in process_count")
        }
    }
}

impl CodeReader {
    pub fn new() -> Self {
        CodeReader {
            state: CodeReaderState::SearchingDirection,
            moves: vec![],
        }
    }

    pub fn read(&mut self, input: char) {
        self.state = match &mut self.state {
            CodeReaderState::SearchingDirection => process_direction(input),
            CodeReaderState::SearchingCount { positive, buffer } => {
                process_count(input, positive, buffer, &mut self.moves)
            }
        }
    }
}

fn main() {
    let code = match fs::read_to_string(INPUT_FILENAME) {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let mut aoc0101_position = CODE_START;
    let mut aoc0101_count = 0;

    let mut aoc0102_position = CODE_START;
    let mut aoc0102_count = 0;

    let mut code_reader = CodeReader::new();
    for char in code.chars() {
        code_reader.read(char);
    }

    for current_move in &code_reader.moves {
        /* AoC 2025 - Day 1 - Part 1 */
        /* Count when zero after rotation */
        aoc0101_position += if current_move.moving_positive {
            current_move.length
        } else {
            -current_move.length
        };
        aoc0101_position %= 100;

        if aoc0101_position == 0 {
            aoc0101_count += 1;
        }

        /* AoC 2025 - Day 1 - Part 2 */
        /* Count every zero hit */
        let mut aoc0102_rotation = current_move.length;
        while aoc0102_rotation > 0 {
            aoc0102_position += if current_move.moving_positive { 1 } else { -1 };
            aoc0102_position %= 100;

            if aoc0102_position == 0 {
                aoc0102_count += 1;
            }
            aoc0102_rotation -= 1;
        }
    }

    println!("The AoC 01-01 code is {}", aoc0101_count);
    println!("The AoC 01-02 code is {}", aoc0102_count);
}
