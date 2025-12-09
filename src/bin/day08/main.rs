use std::collections::HashMap;
use std::collections::HashSet;

// Parse into locations
// find the closest location
// If that location is already in a circuit - add current location to it
// If tht location is not in a circuit - create new circuit - add current and closest location to it
// Store list of circuits

fn main() {
    let input =
        std::fs::read_to_string("src/bin/day08/input.txt").expect("Failed to read input.txt");

    let junctions: Vec<[i64; 3]> = parse_data(&input).collect();

    let mut junction_to_circuit_index: HashMap<[i64; 3], usize> = HashMap::new();

    let mut circuits: Vec<Circuit> = Vec::new();

    let ordered_closest_junctions = get_ordered_closest_junctions(junctions.clone());
    let mut connections = 0;

    let mut seen_pairs: HashSet<([i64; 3], [i64; 3])> = HashSet::new();

    for (junction1, junction2) in ordered_closest_junctions.iter() {
        if seen_pairs.contains(&(*junction1, *junction2))
            || seen_pairs.contains(&(*junction2, *junction1))
        {
            continue;
        }

        seen_pairs.insert((*junction1, *junction2));

        if connections == 1000 {
            break;
        }

        connections += 1;

        let existing_circuit_index_j1 = junction_to_circuit_index.get(junction1).copied();
        let existing_circuit_index_j2 = junction_to_circuit_index.get(junction2).copied();

        match (existing_circuit_index_j1, existing_circuit_index_j2) {
            (Some(circuit_index_j1), Some(circuit_index_j2)) => {
                if circuit_index_j1 == circuit_index_j2 {
                    // in the same circuit - skip
                    continue;
                }

                // Merge circuits - deleting circuit2

                let circuit2_junctions = circuits[circuit_index_j2].junctions.clone();

                circuits[circuit_index_j1].add_junctions(circuit2_junctions.clone());
                // circuits.remove(circuit_index_j2);

                for junction in circuit2_junctions {
                    junction_to_circuit_index.insert(junction, circuit_index_j1);
                }
            }
            (Some(circuit_index_j1), None) => {
                // add to circuit 1
                let circuit = &mut circuits[circuit_index_j1];
                circuit.add_junction(junction2);
                junction_to_circuit_index.insert(*junction2, circuit_index_j1);
            }
            (None, Some(circuit_index_j2)) => {
                // add to circuit w
                let circuit = &mut circuits[circuit_index_j2];
                circuit.add_junction(junction1);
                junction_to_circuit_index.insert(*junction1, circuit_index_j2);
            }
            (None, None) => {
                // New Circuit
                let circuit = Circuit {
                    size: 2,
                    junctions: vec![*junction1, *junction2],
                };
                let new_circuit_index = circuits.len();
                circuits.push(circuit);
                junction_to_circuit_index.insert(*junction1, new_circuit_index);
                junction_to_circuit_index.insert(*junction2, new_circuit_index);
            }
        }
    }

    println!("Total circuits: {:?}", circuits.len());

    println!("Circuit 1 junctions: {:?}", circuits[0].junctions);
    // println!("Circuit 2 junctions: {:?}", circuits[1].junctions);
    // println!("Circuit 3 junctions: {:?}", circuits[2].junctions);

    circuits.sort_by(|a, b| b.size.cmp(&a.size));

    let total = circuits
        .iter()
        .take(3)
        .map(|circuit| circuit.size)
        .product::<usize>();
    println!("Total size of 3 largest circuits: {}", total);
}

struct Circuit {
    junctions: Vec<[i64; 3]>,
    size: usize,
}

impl Circuit {
    fn add_junction(self: &mut Self, junction: &[i64; 3]) {
        self.junctions.push(*junction);
        self.size = self.junctions.len();
    }

    fn add_junctions(self: &mut Self, junctions: Vec<[i64; 3]>) {
        self.junctions.extend(junctions);
        self.size = self.junctions.len();
    }
}

fn parse_data(input: &str) -> impl Iterator<Item = [i64; 3]> {
    return input.lines().map(|line| {
        line.split(",")
            .map(|num| {
                // println!("num: {}", num);
                return num.parse::<i64>().unwrap();
            })
            .collect::<Vec<i64>>()
            .try_into()
            .expect("Failed to parse line into [i64; 3]")
    });
}

fn get_ordered_closest_junctions(junctions: Vec<[i64; 3]>) -> Vec<([i64; 3], [i64; 3])> {
    let mut distances = junctions
        .iter()
        .flat_map(|junction| {
            junctions.iter().map(|other_junction| {
                (
                    *junction,
                    *other_junction,
                    get_distance(junction, other_junction),
                )
            })
        })
        .filter(|(_, _, distance)| *distance > 0.0)
        .collect::<Vec<([i64; 3], [i64; 3], f64)>>();

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    distances
        .iter()
        .map(|(junction, other_junction, _)| (*junction, *other_junction))
        .collect()
}

fn get_distance(junction1: &[i64; 3], junction2: &[i64; 3]) -> f64 {
    return (((junction1[0] - junction2[0]).pow(2)
        + (junction1[1] - junction2[1]).pow(2)
        + (junction1[2] - junction2[2]).pow(2)) as f64)
        .sqrt();
}
