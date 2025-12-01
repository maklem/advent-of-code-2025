use std::fs;

const INPUT_FILENAME: &str = "input.txt";
const CODE_START: i32 = 50;

#[derive(PartialEq)]
enum CodeReaderState {
    SearchingDirection,
    SearchingCount,
    Done,
}

struct Move {
    moving_positive: bool,
    length: i32,
}
struct CodeReader {
    state: CodeReaderState,
    direction_moving_positive: bool,
    length_buffer: String,
}

impl CodeReader {
    pub fn new() -> Self {
        CodeReader {
            state: CodeReaderState::SearchingDirection,
            direction_moving_positive: true,
            length_buffer: String::new(),
        }
    }

    fn process_direction(&mut self, input: char) -> CodeReaderState {
        match input {
            'L' => {
                self.direction_moving_positive = false;
                CodeReaderState::SearchingCount
            }
            'R' => {
                self.direction_moving_positive = true;
                CodeReaderState::SearchingCount
            }
            _ => {
                panic!("unexpected character in process_direction")
            }
        }
    }

    fn process_count(&mut self, input: char) -> CodeReaderState {
        match input {
            '\n' => CodeReaderState::Done,
            '0'..='9' => {
                self.length_buffer.push(input);
                CodeReaderState::SearchingCount
            }
            _ => {
                panic!("unexpected character in process_count")
            }
        }
    }

    pub fn read(&mut self, input: char) -> Option<Move> {
        self.state = match self.state {
            CodeReaderState::SearchingDirection => self.process_direction(input),
            CodeReaderState::SearchingCount => self.process_count(input),
            CodeReaderState::Done => CodeReaderState::Done,
        };

        if self.state == CodeReaderState::Done {
            let length = self.length_buffer.parse::<i32>();
            if length.is_err() {
                panic!(
                    "could not parse int {} error {:?}",
                    self.length_buffer,
                    length.err()
                )
            }
            let next_move = Move {
                moving_positive: self.direction_moving_positive,
                length: length.unwrap(),
            };
            self.length_buffer.clear();
            self.state = CodeReaderState::SearchingDirection;
            return Some(next_move);
        }
        None
    }
}

fn main() {
    let code = match fs::read_to_string(INPUT_FILENAME) {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let mut current_position = CODE_START;
    let mut code_count = 0;

    let mut code_reader = CodeReader::new();
    for char in code.chars() {
        let next_move = code_reader.read(char);

        if let Some(current_move) = next_move {
            current_position += if current_move.moving_positive {
                current_move.length
            } else {
                -current_move.length
            };
            current_position %= 100;

            if current_position == 0 {
                code_count += 1;
            }
        }
    }
    print!("The code is {}", code_count);
}
