use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let input =
        std::fs::read_to_string("src/bin/day07/input.txt").expect("Failed to read input.txt");
    let grid = parse_grid(&input);
    let start_position = get_start_position(&input);

    let splits = track_splits(&grid, start_position);
    println!("Splits: {}", splits);

    let paths = track_paths(&grid, start_position);
    println!("Paths: {}", paths);
}

fn parse_grid(input: &str) -> HashMap<(usize, usize), char> {
    return input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| ((row, col), ch))
        })
        .collect();
}

fn get_start_position(input: &str) -> (usize, usize) {
    let col = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .expect("No start position found");

    return (0, col);
}

fn track_splits(grid: &HashMap<(usize, usize), char>, start_position: (usize, usize)) -> usize {
    let mut splits = 0;
    let mut beam_positions = HashSet::new();
    beam_positions.insert(start_position);

    'outer: loop {
        let mut new_beam_positions = HashSet::new();
        for (row, col) in &beam_positions {
            // Propagate the beam down
            let new_position = (row + 1, *col);
            let new_position_char = grid.get(&new_position);
            match new_position_char {
                Some('.') => {
                    new_beam_positions.insert(new_position);
                }
                Some('^') => {
                    new_beam_positions.insert((row + 1, col - 1));
                    new_beam_positions.insert((row + 1, col + 1));
                    splits += 1;
                }
                None => {
                    break 'outer;
                }
                _ => {
                    panic!("Unexpected character: {}", new_position_char.unwrap());
                }
            }
        }

        beam_positions = new_beam_positions;
    }

    return splits;
}

fn track_paths(grid: &HashMap<(usize, usize), char>, start_position: (usize, usize)) -> usize {
    let mut beam_positions = HashMap::new();
    beam_positions.insert(start_position, 1);

    'outer: loop {
        let mut new_beam_positions = HashMap::new();
        for ((row, col), paths) in &beam_positions {
            // Propagate the beam down
            let new_position = (row + 1, *col);
            let new_position_char = grid.get(&new_position);
            match new_position_char {
                Some('.') => {
                    *new_beam_positions.entry(new_position).or_insert(0) += paths;
                }
                Some('^') => {
                    *new_beam_positions.entry((row + 1, col - 1)).or_insert(0) += paths;
                    *new_beam_positions.entry((row + 1, col + 1)).or_insert(0) += paths;
                }
                None => {
                    break 'outer;
                }
                _ => {
                    panic!("Unexpected character: {}", new_position_char.unwrap());
                }
            }
        }

        beam_positions = new_beam_positions;
    }

    return beam_positions.values().sum();
}
