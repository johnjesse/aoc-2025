use regex::Regex;
use std::collections::HashSet;
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
            let p = part2(&bank);
            println!("Bank {} - Presses: {}", index, p);
            return p;
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
    joltage_diagram: Vec<u64>,
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

    fn toggle_joltage_button(self: &mut Self, button_index: usize) {
        let button: &Button = &self.buttons[button_index];
        for joltage_counter_position in button.positions.clone() {
            self.joltages[joltage_counter_position] = self.joltages[joltage_counter_position] + 1;
        }
    }

    fn is_correct_light_configuration(self: &Self) -> bool {
        return self.lights.iter().enumerate().all(|(index, light)| {
            return *light == self.light_diagram[index];
        });
    }

    fn is_correct_joltage_configuration(self: &Self) -> bool {
        return self.joltages.iter().enumerate().all(|(index, joltage)| {
            return *joltage == self.joltage_diagram[index];
        });
    }

    fn can_still_have_correct_joltage_configuration(self: &Self) -> bool {
        return self.joltages.iter().enumerate().all(|(index, joltage)| {
            return *joltage <= self.joltage_diagram[index];
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

        let joltage_diagram = joltages_regex
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
            joltages: vec![0; joltage_diagram.len()],
            joltage_diagram,
            buttons: buttons,
        };
    }

    fn print_state(self: &Self) {
        println!("Lights: {:?}", self.lights);
        println!("Joltages: {:?}", self.joltages);
        println!("joltage_diagram: {:?}", self.joltage_diagram);
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

fn part2(button_bank: &ButtonBank) -> u64 {
    let mut depth = 0;
    let mut next_sequence: Vec<ButtonBank> = vec![button_bank.clone()];
    let mut visited: HashSet<Vec<u64>> = HashSet::new();
    visited.insert(button_bank.joltages.clone());

    loop {
        depth += 1;

        let sequence = next_sequence.clone();
        next_sequence = vec![];

        for button_bank in sequence {
            for button in button_bank.buttons.clone().iter().enumerate() {
                let mut bank = button_bank.clone();
                bank.toggle_joltage_button(button.0);

                if bank.is_correct_joltage_configuration() {
                    return depth;
                }

                if bank.can_still_have_correct_joltage_configuration()
                    && !visited.contains(&bank.joltages)
                {
                    visited.insert(bank.joltages.clone());
                    next_sequence.push(bank);
                }
            }
        }

        if depth % 10 == 0 {
            println!("Depth: {}", depth);
        }

        if depth > 200 {
            break;
        }
    }

    panic!("No solution found");
}
