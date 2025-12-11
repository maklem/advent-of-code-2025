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

#[derive(Debug)]
struct AoCSolution {
    rows: Vec<Vec<i64>>,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

impl SystemOfLinearEquations {
    pub fn from_machine(machine: &Machine) -> Self {
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

    fn find_row(rows: &Vec<Vec<i64>>, column: usize, skip_rows: usize) -> Option<usize> {
        for row in skip_rows..rows.len() {
            if rows[row][column] != 0 {
                return Some(row);
            }
        }
        None
    }

    fn eliminate_column(
        rows: &mut Vec<Vec<i64>>,
        source_row: usize,
        column: usize,
        target_row: usize,
    ) {
        let factor_source = rows[target_row][column];
        let factor_target = rows[source_row][column];

        let factor = if factor_target * rows[target_row][column + 1]
            - factor_source * rows[source_row][column + 1]
            > 0
        {
            1
        } else {
            -1
        };
        for col in 0..rows[target_row].len() {
            rows[target_row][col] = factor
                * (factor_target * rows[target_row][col] - factor_source * rows[source_row][col]);
        }
    }

    /// https://en.wikipedia.org/wiki/Gaussian_elimination
    pub fn gaussian_elimination(&self) -> AoCSolution {
        let mut rows = self.rows.clone();
        let mut dependents = vec![];
        let mut independents = vec![];

        let column_count = self.rows.first().unwrap().len();
        let row_count = self.rows.len();
        for index in 0..column_count - 1 {
            let current_row = dependents.len();
            let column = dependents.len() + independents.len();
            if let Some(row) = SystemOfLinearEquations::find_row(&rows, column, current_row) {
                if row != current_row {
                    rows.swap(row, current_row);
                }

                for row in current_row + 1..row_count {
                    SystemOfLinearEquations::eliminate_column(&mut rows, current_row, column, row);
                }

                dependents.push(index);
            } else {
                independents.push(index);
            }
        }
        AoCSolution {
            rows: rows,
            dependents,
            independents,
        }
    }
}

impl AoCSolution {
    fn test_at(
        &self,
        parameters: &mut Vec<i64>,
        independent_parameters: &[i64],
        column: usize,
    ) -> Option<i64> {
        let row = column - independent_parameters.len();

        if self.independents.contains(&column) {
            parameters[column] = independent_parameters[0];
            self.test_at(parameters, &independent_parameters[1..], column - 1)
        } else if self.dependents.contains(&column) {
            let intermediate: i64 = parameters
                .iter()
                .enumerate()
                .map(|(column, value)| self.rows[row][column] * value)
                .sum();
            if intermediate % self.rows[row][column] == 0 {
                parameters[column] = -intermediate / self.rows[row][column];
                if parameters[column] >= 0 {
                    if row == 0 {
                        Some(parameters.iter().sum::<i64>() + 1)
                    } else {
                        self.test_at(parameters, &independent_parameters, column - 1)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn test_parameters(&self, independent_values: &[i64]) -> Option<i64> {
        let parameter_count = self.independents.len() + self.dependents.len();

        let mut parameters = vec![0; parameter_count];
        parameters.push(-1);

        self.test_at(&mut parameters, independent_values, parameter_count - 1)
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

fn process_joltage_layer(
    machine: &Machine,
    sol: &AoCSolution,
    index: usize,
    independent_values: &mut Vec<i64>,
) -> Option<i64> {
    let maximum = machine.joltages.values.iter().max().unwrap();
    let mut result = None;

    if index == independent_values.len() {
        result = sol.test_parameters(independent_values);
    } else {
        for current in 0..*maximum {
            independent_values[index] = current;

            let intermediate_result =
                process_joltage_layer(machine, sol, index + 1, independent_values);

            result = if let Some(new_value) = intermediate_result {
                if let Some(old_value) = result
                    && old_value < new_value
                {
                    result
                } else {
                    intermediate_result
                }
            } else {
                result
            }
        }
    }
    result
}

fn find_minimal_button_presses_for_joltage(machine: &Machine) -> i64 {
    let eqn = SystemOfLinearEquations::from_machine(machine);
    let sol = eqn.gaussian_elimination();

    let mut indeps = vec![0; sol.independents.len()];

    if let Some(value) = process_joltage_layer(machine, &sol, 0, &mut indeps) {
        value
    } else {
        println!("{:?}", sol);
        0
    }
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
    use std::ptr::eq;

    use crate::SystemOfLinearEquations;

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

    #[test]
    fn SystemOfLinearEquations__given_unit_matrix__solves_unchanged() {
        let matrix = vec![vec![1, 0, 0, 1], vec![0, 1, 0, 1], vec![0, 0, 1, 1]];

        let mut eqns = SystemOfLinearEquations {
            rows: matrix.clone(),
        };
        let solution = eqns.gaussian_elimination();

        assert_eq!(solution.rows, matrix);
    }

    #[test]
    fn SystemOfLinearEquations__given_flipped_unit_matrix__solves_to_unit_matrix() {
        let matrix_expected = vec![vec![1, 0, 0, 1], vec![0, 1, 0, 1], vec![0, 0, 1, 1]];
        let matrix = vec![vec![0, 0, 1, 1], vec![0, 1, 0, 1], vec![1, 0, 0, 1]];

        let mut eqns = SystemOfLinearEquations { rows: matrix };
        let solution = eqns.gaussian_elimination();

        assert_eq!(solution.rows, matrix_expected);
    }

    #[test]
    fn SystemOfLinearEquations__given_lower_triangle_matrix__solves_to_unit_matrix() {
        let matrix_expected = vec![vec![1, 0, 0, 1], vec![0, 1, 0, 1], vec![0, 0, 1, 1]];
        let matrix = vec![vec![1, 0, 0, 1], vec![1, 1, 0, 2], vec![1, 1, 1, 3]];

        let mut eqns = SystemOfLinearEquations { rows: matrix };
        let solution = eqns.gaussian_elimination();

        assert_eq!(solution.rows, matrix_expected);
    }
}
