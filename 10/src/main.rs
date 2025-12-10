use std::{cmp::min, iter::zip};

struct LightPattern {
    pattern: Vec<bool>,
}

impl LightPattern {
    fn from_str(line: &str) -> Self {
        Self {
            pattern: line
                .chars()
                .filter(|x| *x == '.' || *x == '#')
                .map(|x| x == '#')
                .collect(),
        }
    }

    fn from_len(len: usize) -> Self {
        Self {
            pattern: vec![false; len],
        }
    }

    fn len(&self) -> usize {
        self.pattern.len()
    }

    fn apply_button(&mut self, button: &Button) {
        for index in &button.connected_lights {
            self.pattern[*index] = !self.pattern[*index];
        }
    }
}

impl PartialEq for LightPattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern.len() == other.pattern.len()
            && zip(&self.pattern, &other.pattern).all(|(a, b)| a == b)
    }
}

struct Button {
    connected_lights: Vec<usize>,
}

impl Button {
    fn from_str(line: &str) -> Self {
        Self {
            connected_lights: line[1..line.len() - 1]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}

struct Joltages {
    _values: Vec<usize>,
}

impl Joltages {
    fn from_str(line: &str) -> Self {
        Self {
            _values: line[1..line.len() - 1]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}

struct Machine {
    pattern: LightPattern,
    buttons: Vec<Button>,
    _joltages: Joltages,
}

fn to_machine(line: &str) -> Machine {
    let elem: Vec<_> = line.split(" ").collect();
    Machine {
        pattern: LightPattern::from_str(elem[0]),
        buttons: elem[1..elem.len() - 1]
            .iter()
            .map(|s| Button::from_str(s))
            .collect(),
        _joltages: elem.last().map(|s| Joltages::from_str(s)).unwrap(),
    }
}

fn combinations(count: usize, slots: usize, max_count: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    if count == 0 {
        result.push(vec![0; slots]);
    } else {
        for padding in 0..=(slots - count) {
            let mut local_pattern = vec![0; padding];
            for current_count in 1..=min(count, max_count) {
                local_pattern.push(current_count);
                for combination in combinations(count - current_count, slots - padding - 1, max_count) {
                    let mut sub_pattern = local_pattern.clone();
                    combination.iter().for_each(|y| sub_pattern.push(*y));
                    result.push(sub_pattern);
                }
            }
        }
    }
    result
}

fn find_minimal_button_presses_for_pattern(machine: Machine) -> usize {
    let button_count = machine.buttons.len();
    for available_buttons in 1..button_count {
        for combination in combinations(available_buttons, button_count, 1) {
            let mut pattern = LightPattern::from_len(machine.pattern.len());
            for button_press in combination
                .iter()
                .enumerate()
                .filter(|(_, active)| **active > 0)
                .map(|(i, _)| i)
            {
                pattern.apply_button(&machine.buttons[button_press]);
            }
            if pattern == machine.pattern {
                return available_buttons;
            }
        }
    }
    panic!("No suitable pattern found");
}

fn main() {
    let start = std::time::Instant::now();
    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let machines: Vec<_> = code.lines().map(to_machine).collect();

    let mut button_presses = 0;
    for machine in machines {
        button_presses += find_minimal_button_presses_for_pattern(machine);
    }
    println!("Buttons pressed: {}", button_presses);

    println!(
        "evaluation took {} ms",
        start.elapsed().as_nanos() as f64 / 1e6
    )
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::combinations;

    #[test]
    fn combinations__for_0_slots__returns_list_of_false() {
        assert_eq!(vec![vec![0; 5]], combinations(0, 5, 1))
    }

    #[test]
    fn combinations__for_entries_eq_slots__returns_list_of_true() {
        assert_eq!(vec![vec![1; 5]], combinations(5, 5, 1))
    }
}
