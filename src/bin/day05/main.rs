fn main() {
    let input =
        std::fs::read_to_string("src/bin/day05/input.txt").expect("Failed to read input.txt");

    let Data { mut ranges, ids } = parse_to_ranges_and_ids(&input);

    let fresh_count = ids.iter().filter(|&&id| is_fresh_id(&ranges, id)).count();
    println!("Total Fresh part 1: {}", fresh_count);

    ranges.sort_by_key(|r| r.start);
    let compacted = compact_ranges(ranges);

    let total: i64 = compacted.iter().map(|r| r.end + 1 - r.start).sum();
    println!("Total Fresh part 2: {}", total);
}

#[derive(Clone, Debug)]
struct Range {
    start: i64,
    end: i64,
}

struct Data {
    ranges: Vec<Range>,
    ids: Vec<i64>,
}

fn parse_to_ranges_and_ids(input: &str) -> Data {
    let (ranges_input, ids_input) = input.split_once("\n\n").unwrap();

    let ranges = ranges_input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect();

    let ids = ids_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    Data { ranges, ids }
}

fn is_fresh_id(ranges: &[Range], id: i64) -> bool {
    ranges.iter().any(|r| id >= r.start && id <= r.end)
}

fn compact_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut iter = ranges.into_iter();
    let mut result = vec![iter.next().unwrap()];

    for range in iter {
        let last = result.last().unwrap();
        if range.start <= last.end + 1 {
            // Merge: extend the last range
            let last = result.last_mut().unwrap();
            last.end = last.end.max(range.end);
        } else {
            result.push(range);
        }
    }
    result
}
