use regex::Regex;
use std::collections::HashMap;
fn main() {
    let input =
        std::fs::read_to_string("src/bin/day11/input.txt").expect("Failed to read input.txt");

    let racks = parse_to_server_racks(&input);

    // Part 1
    let paths = find_paths_simple(&racks, &"you".to_string());
    println!("Paths: {}", paths);

    // Part 2
    let mut cache = HashMap::new();
    let paths = find_paths_recursive(&racks, &"you".to_string(), &[], vec![], &mut cache);
    println!("Paths: {}", paths);

    let paths = find_paths_recursive(
        &racks,
        &"svr".to_string(),
        &["dac".to_string(), "fft".to_string()],
        vec![false, false],
        &mut cache,
    );
    println!("Paths: {}", paths);
}

#[derive(Clone)]
struct ServerRack {
    outputs: Vec<String>,
}

fn parse_to_server_racks(input: &String) -> HashMap<String, ServerRack> {
    let mut racks: HashMap<String, ServerRack> = HashMap::new();

    let rack_id_regex = Regex::new(r"[a-z][a-z][a-z]").unwrap();

    for line in input.lines() {
        let ids = rack_id_regex
            .find_iter(line)
            .map(|m| m.as_str().to_string())
            .collect::<Vec<String>>();

        let (rack_id, outputs) = ids.split_first().unwrap();

        racks.insert(
            rack_id.to_string(),
            ServerRack {
                outputs: outputs.to_vec(),
            },
        );
    }

    return racks;
}

fn find_paths_simple(racks: &HashMap<String, ServerRack>, start_rack_name: &String) -> usize {
    let start = racks.get(start_rack_name).unwrap();

    let mut stack = Vec::new();
    stack.push(start);
    let mut paths = 1;

    loop {
        let next_rack = stack.pop().unwrap();

        let rack_outputs = next_rack.outputs.len();
        for output in next_rack.outputs.clone() {
            if output == "out" {
                continue;
            }

            let output_rack = racks.get(&output).unwrap();
            stack.push(output_rack);
        }

        if rack_outputs > 1 {
            paths += rack_outputs - 1;
        }

        if stack.len() == 0 {
            break;
        }

        if paths % 1000 == 0 {
            println!("Paths: {}", paths);
        }
    }

    return paths;
}

fn find_paths_recursive(
    racks: &HashMap<String, ServerRack>,
    start_rack_name: &String,
    required_racks: &[String],
    seen_required: Vec<bool>,
    cache: &mut HashMap<(String, Vec<bool>), usize>,
) -> usize {
    let cache_key = (start_rack_name.clone(), seen_required.clone());

    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let start: &ServerRack = racks.get(start_rack_name).unwrap();

    let outputs = start.outputs.clone();

    let mut new_seen = seen_required.clone();

    for (i, req) in required_racks.iter().enumerate() {
        if start_rack_name == req {
            new_seen[i] = true;
        }
    }

    let paths = outputs
        .iter()
        .map(|output| {
            if output == "out" {
                // Only count if ALL required racks were seen
                if new_seen.iter().all(|&s| s) { 1 } else { 0 }
            } else {
                return find_paths_recursive(
                    racks,
                    &output,
                    required_racks,
                    new_seen.clone(),
                    cache,
                );
            }
        })
        .sum();

    cache.insert(cache_key, paths);
    return paths;
}
