use std::collections::HashSet;

fn main() {
    let presents_input =
        std::fs::read_to_string("src/bin/day12/presents.txt").expect("Failed to read presents.txt");

    let areas_input =
        std::fs::read_to_string("src/bin/day12/areas.txt").expect("Failed to read areas.txt");

    let presents = parse_presents(&presents_input);
    let areas = parse_areas(&areas_input);

    let trivially_packable_areas = areas
        .iter()
        .filter(|area| is_area_trivially_packable(&area))
        .count();
    let impossible_to_pack_areas = areas
        .iter()
        .filter(|area| is_area_impossible_to_pack(&area))
        .count();
    println!("Trivially packable areas: {}", trivially_packable_areas);
    println!("Impossible to pack areas: {}", impossible_to_pack_areas);
    let total_areas = areas.len();
    println!("Total areas: {}", total_areas);
}

#[derive(Clone, Debug)]
struct Present {
    id: usize,
    grid: HashSet<(usize, usize)>,
}

impl Present {
    fn rotate_clockwise_90(self: &mut Self) {
        let mut new_grid = HashSet::new();
        for point in &self.grid {
            match point {
                (0, 0) => {
                    new_grid.insert((0, 2));
                }
                (0, 1) => {
                    new_grid.insert((1, 2));
                }
                (0, 2) => {
                    new_grid.insert((2, 2));
                }
                (2, 0) => {
                    new_grid.insert((0, 0));
                }
                (1, 0) => {
                    new_grid.insert((0, 1));
                }
                (2, 2) => {
                    new_grid.insert((2, 0));
                }
                (2, 1) => {
                    new_grid.insert((1, 0));
                }
                (1, 2) => {
                    new_grid.insert((2, 1));
                }
                (1, 1) => {
                    new_grid.insert((1, 1));
                }
                _ => {
                    panic!("Invalid point: {:?}", point);
                }
            }
        }
        self.grid = new_grid;
    }

    fn from_lines(lines: &Vec<String>) -> Self {
        let mut id = 0;
        let mut grid = HashSet::new();

        lines.iter().enumerate().for_each(|(line_index, line)| {
            match line_index {
                0 => {
                    id = line.split(":").nth(0).unwrap().parse::<usize>().unwrap();
                }
                1 | 2 | 3 => {
                    line.chars().enumerate().for_each(|(column_index, c)| {
                        if c == '#' {
                            grid.insert((line_index - 1, column_index));
                        }
                    });
                }
                _ => {
                    panic!("Invalid line index: {}", line_index);
                }
            };
        });

        return Present { id, grid };
    }

    fn print_grid(self: &Self) {
        for row in 0..3 {
            for col in 0..3 {
                if self.grid.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

#[derive(Clone, Debug)]
struct Area {
    height: usize,
    width: usize,
    presents: Vec<(usize, usize)>,
}

fn parse_presents(input: &str) -> Vec<Present> {
    let mut line_groups = Vec::new();

    let mut current_group = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            line_groups.push(current_group);
            current_group = Vec::new();
        } else {
            current_group.push(line.to_string());
        }
    }

    // Add the last group if it exists
    if !current_group.is_empty() {
        line_groups.push(current_group);
    }

    return line_groups
        .iter()
        .map(|group| Present::from_lines(group))
        .collect::<Vec<Present>>();
}

fn parse_areas(input: &str) -> Vec<Area> {
    return input
        .lines()
        .map(|line| {
            let (dims, presents) = line.split_once(":").unwrap();

            let (height, width) = dims.split_once("x").unwrap();
            let height = height.parse::<usize>().unwrap();
            let width = width.parse::<usize>().unwrap();

            let presents = presents
                .trim()
                .split(" ")
                .enumerate()
                .map(|(present_id, num_presents)| {
                    return (present_id, num_presents.parse::<usize>().unwrap());
                })
                .collect::<Vec<(usize, usize)>>();

            return Area {
                height,
                width,
                presents,
            };
        })
        .collect::<Vec<Area>>();
}

// Technically this is not correct, but it's a good enough approximation for now
fn is_area_trivially_packable(area: &Area) -> bool {
    let area_size = area.height * area.width;
    let maximum_present_area = area.presents.iter().map(|(_, count)| count).sum::<usize>() * 9;
    return area_size >= maximum_present_area;
}

fn is_area_impossible_to_pack(area: &Area) -> bool {
    let area_size = area.height * area.width;
    let minimum_present_area = area
        .presents
        .iter()
        .map(|(present_index, count)| match present_index {
            0 => count * 7,
            1 => count * 7,
            2 => count * 7,
            3 => count * 5,
            4 => count * 7,
            5 => count * 6,
            _ => panic!("Invalid present index: {}", present_index),
        })
        .sum::<usize>();
    return area_size < minimum_present_area;
}
