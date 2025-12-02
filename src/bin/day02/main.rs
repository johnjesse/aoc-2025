struct Range {
    start: i64,
    end: i64,
}

fn main() {
    let input =
        std::fs::read_to_string("src/bin/day02/input.txt").expect("Failed to read input.txt");

    let ranges = parse_ranges(&input);
    run(&ranges);
}

fn parse_ranges(input: &str) -> Vec<Range> {
    let ranges: Vec<Range> = input
        .split(",")
        .map(|item| {
            let (start_raw, end_raw) = item
                .split_once("-")
                .expect("No separator found for range parsing");

            let start = start_raw.parse::<i64>().expect("Failed to parse sizeStr");
            let end = end_raw.parse::<i64>().expect("Failed to parse sizeStr");

            return Range { start, end };
        })
        .collect();

    return ranges;
}

fn is_invalid_id_part_2(id: i64) -> bool {
    let id_str = id.to_string();
    let length = id_str.len();
    let end = length.div_ceil(2);

    if length < 2 {
        return false;
    }

    for i in 1..=end {
        if length % i != 0 {
            continue;
        }

        let section: String = id_str.chars().take(i).collect();
        let n_times = length / i;
        let repeated_section = section.repeat(n_times);

        if repeated_section == id_str {
            return true;
        }
    }

    return false;
}

// fn is_invalid_id(id: i64) -> bool {
//     let str_id = id.to_string();

//     if str_id.len() % 2 != 0 {
//         return false;
//     }
//     let mid = str_id.len() / 2;
//     let (first_half, second_half) = str_id.split_at(mid);

//     return first_half == second_half;
// }

fn get_invalid_ids_in_range(range: &Range) -> Vec<i64> {
    let &Range { start, end } = range;
    let mut invalid_ids = Vec::new();

    for id in start..=end {
        if is_invalid_id_part_2(id) {
            invalid_ids.push(id)
        }
    }

    return invalid_ids;
}

fn run(ranges: &Vec<Range>) -> () {
    let mut all_invalid_ids = Vec::new();

    for range in ranges {
        all_invalid_ids.extend(get_invalid_ids_in_range(range));
    }

    let total = all_invalid_ids.iter().sum::<i64>();

    println!("Total {}", total);
    return;
}
