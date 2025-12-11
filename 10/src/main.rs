use std::{
    cmp::{max, min},
    iter::zip,
};

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

#[derive(Clone)]
struct Joltages {
    values: Vec<i64>,
}

impl Joltages {
    fn from_str(line: &str) -> Self {
        Self {
            values: line[1..line.len() - 1]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    fn apply_button(&mut self, button: &Button, count: usize) {
        for index in &button.connected_lights {
            self.values[*index] -= count as i64;
        }
    }

    fn is_perfect(&self) -> bool {
        self.values.iter().all(|v| *v == 0)
    }
}

struct Machine {
    pattern: LightPattern,
    buttons: Vec<Button>,
    joltages: Joltages,
}

fn to_machine(line: &str) -> Machine {
    let elem: Vec<_> = line.split(" ").collect();
    Machine {
        pattern: LightPattern::from_str(elem[0]),
        buttons: elem[1..elem.len() - 1]
            .iter()
            .map(|s| Button::from_str(s))
            .collect(),
        joltages: elem.last().map(|s| Joltages::from_str(s)).unwrap(),
    }
}

struct SystemOfLinearEquations {
    rows: Vec<Vec<i64>>,
}

impl SystemOfLinearEquations {
    fn from_machine(machine: &Machine) -> Self {
        Self {
            rows: machine
                .joltages
                .values
                .iter()
                .enumerate()
                .map(|(index, jolts)| {
                    let mut row: Vec<_> = machine
                        .buttons
                        .iter()
                        .map(|btn| {
                            if btn.connected_lights.contains(&index) {
                                1
                            } else {
                                0
                            }
                        })
                        .collect();
                    row.push(*jolts);
                    row
                })
                .collect(),
        }
    }

    /// https://en.wikipedia.org/wiki/Gaussian_elimination
    fn gaussian_elimination(&mut self, row: usize, column: usize) {

    }
}

fn combinations(count: usize, slots: usize, max_count: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    if count == 0 {
        result.push(vec![0; slots]);
    } else {
        let max_padding = slots - count.div_ceil(max_count);
        for padding in 0..=max_padding {
            let min_count = max(1, count - min(count, max_count * (slots - padding - 1)));
            let max_count = min(count, max_count);
            for current_count in min_count..=max_count {
                let mut local_pattern = vec![0; padding];
                local_pattern.push(current_count);
                for combination in
                    combinations(count - current_count, slots - padding - 1, max_count)
                {
                    let mut sub_pattern = local_pattern.clone();
                    combination.iter().for_each(|y| sub_pattern.push(*y));
                    result.push(sub_pattern);
                }
            }
        }
    }
    if result.iter().any(|x| x.len() != slots) {
        panic!(
            "SIZE MISMATCH!! {} != {} (count: {})",
            result.len(),
            slots,
            count
        );
    }

    result
}

fn find_minimal_button_presses_for_pattern(machine: &Machine) -> usize {
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

fn find_minimal_button_presses_for_joltage(machine: &Machine) -> usize {
    let mut eqn = SystemOfLinearEquations::from_machine(machine);
    eqn.gaussian_elimination();
    0
}

fn main() {
    let start = std::time::Instant::now();
    let code = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(err) => panic!("Could not read file: {}", err),
    };
    let machines: Vec<_> = code.lines().map(to_machine).collect();

    let mut button_presses = 0;
    let mut joltage_presses = 0;
    for machine in machines {
        button_presses += find_minimal_button_presses_for_pattern(&machine);
        joltage_presses += find_minimal_button_presses_for_joltage(&machine);
        print!(".")
    }
    println!("[Part 1] Buttons pressed for pattern: {}", button_presses);
    println!("[Part 2] Buttons pressed for joltage: {}", joltage_presses);

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

    #[test]
    fn combinations__for_entries_2x_slots__returns_list_of_two() {
        assert_eq!(vec![vec![2; 5]], combinations(10, 5, 2))
    }

    #[test]
    fn combinations__for_entries_5x_slots__returns_list_of_five() {
        assert_eq!(vec![vec![5; 5]], combinations(25, 5, 5))
    }
}
