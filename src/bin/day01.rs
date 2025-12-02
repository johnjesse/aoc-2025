enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    steps: i32,
}

fn main() {
    let input = std::fs::read_to_string("src/bin/input.txt").expect("Failed to read input.txt");

    let rotations = parse_rotations(&input);
    part1(&rotations);
    part2(&rotations);
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    let rotations: Vec<Rotation> = input
        .split("\n")
        .map(|item| {
            let (dir, size_str) = item.split_at(1);

            let direction = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction: {}", dir),
            };

            let steps = size_str.parse::<i32>().expect("Failed to parse sizeStr");

            return Rotation { direction, steps };
        })
        .collect();

    return rotations;
}

fn rotate_value(value: i32, direction: &Direction) -> i32 {
    match direction {
        Direction::Left => {
            if value == 0 {
                return 99;
            } else {
                return value - 1;
            }
        }
        Direction::Right => {
            if value == 99 {
                return 0;
            } else {
                return value + 1;
            }
        }
    }
}

fn part1(rotations: &Vec<Rotation>) -> () {
    let mut current_value = 50;
    let mut num_zeros: i32 = 0;

    for Rotation { direction, steps } in rotations {
        for _i in 0..*steps {
            current_value = rotate_value(current_value, direction);
        }

        // println!("Current Value {}", current_value);

        if current_value == 0 {
            num_zeros += 1;
        }
    }

    println!("Password {}", num_zeros);
    return;
}

fn part2(rotations: &Vec<Rotation>) -> () {
    let mut current_value = 50;
    let mut num_zeros: i32 = 0;

    for Rotation { direction, steps } in rotations {
        for _i in 0..*steps {
            current_value = rotate_value(current_value, direction);
            if current_value == 0 {
                num_zeros += 1;
            }
        }
        // println!("Current Value {}", current_value);
    }

    println!("Password {}", num_zeros);
    return;
}
