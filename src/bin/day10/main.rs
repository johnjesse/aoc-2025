use regex::Regex;
fn main() {
    let input =
        std::fs::read_to_string("src/bin/day10/input.txt").expect("Failed to read input.txt");

    for bank in parse_data(&input) {
        bank.print_state();
    }

    // let part_1_presses = &parse_data(&input)
    //     .enumerate()
    //     .map(|(index, bank)| {
    //         let p = part1(&bank);
    //         println!("Bank {} - Presses: {}", index, p);
    //         return p;
    //     })
    //     .sum::<u64>();
    // println!("Part 1 Presses: {}", part_1_presses);

    let part_2_presses = &parse_data(&input)
        .enumerate()
        .map(|(index, bank)| {
            let p = solve_button_bank_joltages(&bank);
            match p {
                Ok(total) => {
                    // println!("Total presses: {}, index: {}", total, index);
                    return total;
                }
                Err(msg) => {
                    println!("Cannot solve: {}, index: {}", msg, index);
                    return 0;
                }
            }
        })
        .sum::<u64>();
    println!("Part 2 Presses: {}", part_2_presses);
}

#[derive(Clone)]
struct ButtonBank {
    lights: Vec<bool>,
    joltages: Vec<u64>,
    buttons: Vec<Button>,
    light_diagram: Vec<bool>,
}

#[derive(Debug, Clone)]
struct Button {
    positions: Vec<usize>,
}

impl ButtonBank {
    fn toggle_light_button(self: &mut Self, button_index: usize) {
        let button: &Button = &self.buttons[button_index];
        for light_position in button.positions.clone() {
            self.lights[light_position] = !self.lights[light_position];
        }
    }

    fn is_correct_light_configuration(self: &Self) -> bool {
        return self.lights.iter().enumerate().all(|(index, light)| {
            return *light == self.light_diagram[index];
        });
    }

    fn from_line(line: &str) -> Self {
        let diagram_regex = Regex::new(r"^\[([.#]+)\]").unwrap();
        let button_regex = Regex::new(r"\(([0-9,]+)\)").unwrap();
        let joltages_regex = Regex::new(r"\{([\d,]+)\}$").unwrap();
        let diagram = diagram_regex
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<bool>>();

        let buttons = button_regex
            .captures_iter(line)
            .map(|m| {
                let positions = m
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(",")
                    .map(|pos| pos.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                return Button {
                    positions: positions,
                };
            })
            .collect::<Vec<Button>>();

        let joltages = joltages_regex
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(",")
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        return ButtonBank {
            lights: vec![false; diagram.len()],
            light_diagram: diagram,
            joltages: joltages,
            buttons: buttons,
        };
    }

    fn print_state(self: &Self) {
        println!("Lights: {:?}", self.lights);
        println!("Joltages: {:?}", self.joltages);
        println!("Buttons: {:?}", self.buttons);
        println!("Light diagram: {:?}", self.light_diagram);
    }
}

fn parse_data(input: &str) -> impl Iterator<Item = ButtonBank> {
    return input.lines().map(|line| ButtonBank::from_line(line));
}

fn part1(button_bank: &ButtonBank) -> u64 {
    let mut depth = 0;
    let mut next_sequence: Vec<ButtonBank> = vec![button_bank.clone()];

    loop {
        depth += 1;

        let sequence = next_sequence.clone();
        next_sequence = vec![];

        for button_bank in sequence {
            for button in button_bank.buttons.clone().iter().enumerate() {
                let mut bank = button_bank.clone();
                bank.toggle_light_button(button.0);

                if bank.is_correct_light_configuration() {
                    return depth;
                }

                next_sequence.push(bank);
            }
        }

        if depth > 20 {
            break;
        }
    }

    panic!("No solution found");
}

fn create_matrix_from_button_bank(button_bank: &ButtonBank) -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> =
        vec![vec![0.0; button_bank.buttons.len() + 1]; button_bank.joltages.len()];

    for (button_position, button) in button_bank.buttons.clone().iter().enumerate() {
        for position in button.positions.clone() {
            matrix[position][button_position] = 1.0;
        }
    }

    for (joltage_position, joltage) in button_bank.joltages.clone().iter().enumerate() {
        matrix[joltage_position][button_bank.buttons.len()] = *joltage as f64;
    }

    return matrix;
}

fn print_matrix(matrix: &Vec<Vec<f64>>) {
    for row in matrix {
        for val in row {
            print!("{:8.2} ", val);
        }
        println!();
    }
}

fn forward_elimination(matrix: &mut Vec<Vec<f64>>) -> () {
    let rows = matrix.len();
    if rows == 0 {
        return;
    }

    let mut pivot_row = 0;

    let cols = matrix[0].len();
    let num_params = cols - 1;

    for col in 0..num_params {
        // Step 1: find the row with the largest abs value in this column, below the pivot row
        let mut max_row = pivot_row;
        let mut max_val = matrix[pivot_row][col].abs();

        for row in (pivot_row + 1)..rows {
            if matrix[row][col].abs() > max_val {
                max_val = matrix[row][col].abs();
                max_row = row;
            }
        }

        // Step 2:If max value is ~0 then skip - no pivot
        if max_val < 1e-10 {
            continue;
        }

        // Step 3: Swat the pivot and max rows
        if max_row != pivot_row {
            matrix.swap(pivot_row, max_row);
        }

        // Step 4 : Eliminate entries in this column below the pivot row
        for row in (pivot_row + 1)..rows {
            if matrix[row][col].abs() > 1e-10 {
                let factor = matrix[row][col] / matrix[pivot_row][col];

                // Subtract factor * pivot_row from this row
                for c in col..cols {
                    matrix[row][c] -= factor * matrix[pivot_row][c];
                }
            }
        }
        // Repeat

        pivot_row += 1;

        // If we've used all rows, we're done
        if pivot_row >= rows {
            break;
        }
    }
}

fn backward_substitution(matrix: &mut Vec<Vec<f64>>) -> Vec<f64> {
    let rows = matrix.len();
    if rows == 0 {
        return vec![];
    }

    let cols = matrix[0].len();
    let num_params = cols - 1;

    let mut solution = vec![0.0; num_params];

    // Work from bottom row to top
    for row in (0..rows).rev() {
        // Step 1: Find the pivot column (first non-zero entry in this row)
        let mut pivot_col: Option<usize> = None;
        for col in 0..num_params {
            if matrix[row][col].abs() > 1e-10 {
                pivot_col = Some(col);
                break;
            }
        }

        // Step 2: If no pivot found, skip this row
        let pivot_col = match pivot_col {
            Some(col) => col,
            None => continue,
        };

        // Step 3: Start with the target value (rightmost column)
        let mut value = matrix[row][num_params];

        // Step 4: Subtract known variables (columns to the right of pivot)
        for col in (pivot_col + 1)..num_params {
            value -= matrix[row][col] * solution[col];
        }

        // Step 5: Solve for this variable
        solution[pivot_col] = value / matrix[row][pivot_col];
    }

    return solution;
}

fn is_under_determined(matrix: &Vec<Vec<f64>>) -> bool {
    let rows = matrix.len();
    let num_params = matrix[0].len() - 1;

    return rows < num_params;
}

fn find_free_variables(matrix: &Vec<Vec<f64>>, num_params: usize) -> Vec<usize> {
    let mut pivot_cols: Vec<usize> = Vec::new();

    for row in matrix {
        for col in 0..num_params {
            if row[col].abs() > 1e-10 {
                if !pivot_cols.contains(&col) {
                    pivot_cols.push(col);
                }
                break; // only first non-zero is pivot for this row
            }
        }
    }

    // Free = all columns NOT in pivot_cols
    (0..num_params)
        .filter(|col| !pivot_cols.contains(col))
        .collect()
}

fn solve_under_determined(matrix: &Vec<Vec<f64>>, button_bank: &ButtonBank) -> u64 {
    let num_params = button_bank.buttons.len();

    let free_cols = find_free_variables(&matrix, num_params);

    let max_joltage = button_bank.joltages.iter().max().unwrap_or(&0);

    match search_free_variables(&matrix, &free_cols, *max_joltage) {
        Some(total) => total,
        None => panic!("No valid solution found"),
    }
}

fn solve_button_bank_joltages(button_bank: &ButtonBank) -> Result<u64, String> {
    // Step 1: Build the matrix
    let mut matrix = create_matrix_from_button_bank(button_bank);

    // println!("Initial matrix:");
    // print_matrix(&matrix);

    // Step 2: Forward elimination
    forward_elimination(&mut matrix);

    if is_under_determined(&matrix) {
        return Err(String::from("Under-determined"));
    }

    // println!("\nAfter forward elimination:");
    // print_matrix(&matrix);

    // Step 3: Back substitution
    let solution = backward_substitution(&mut matrix);

    // println!("Solution (button presses): {:?}", solution);

    // Sum up the button presses
    let total: u64 = solution.iter().map(|&x| x.round() as u64).sum();

    // println!("Total presses: {}", total);

    return Ok(total);
}
