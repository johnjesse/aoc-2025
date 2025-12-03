fn main() {
    let input =
        std::fs::read_to_string("src/bin/day03/input.txt").expect("Failed to read input.txt");

    let joltages: Vec<Vec<u64>> = parse_banks(&input).collect();

    let total_part_1 = joltages
        .iter()
        .map(|joltages| get_largest_joltage(joltages.clone(), 2))
        .sum::<u64>();

    let total_part_2 = joltages
        .iter()
        .map(|joltages| get_largest_joltage(joltages.clone(), 12))
        .sum::<u64>();
    println!("Totals 1: {} 2: {}", total_part_1, total_part_2)
}

fn parse_banks(input: &str) -> impl Iterator<Item = Vec<u64>> {
    return input.lines().map(|line| {
        line.chars()
            .map(|char| char.to_digit(10).unwrap() as u64)
            .collect()
    });
}

fn get_largest_joltage(joltages: Vec<u64>, battery_size: usize) -> u64 {
    let mut first_value: u64 = 0;
    let mut first_value_index = 0;

    for (index, value) in joltages.iter().enumerate() {
        if index > joltages.len() - battery_size {
            continue;
        }

        if value > &first_value {
            first_value = *value;
            first_value_index = index
        }
    }

    if battery_size == 1 {
        return first_value;
    }

    let second_value: u64 =
        get_largest_joltage(joltages[first_value_index + 1..].to_vec(), battery_size - 1);

    let joltage = first_value.to_string() + &second_value.to_string();
    return joltage.parse::<u64>().unwrap();
}
