use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let input =
        std::fs::read_to_string("src/bin/day04/input.txt").expect("Failed to read input.txt");

    let mut grid = parse_to_grid(&input);

    let mut total_movable_paper: i32 = 0;

    // part 1
    // for row in 0..height {
    //     for col in 0..width {
    //         if is_movable_paper(&content, row, col) {
    //             // println!("Movable paper at {} {}", row, col);
    //             total_movable_paper += 1;
    //         }
    //     }
    // }

    loop {
        let NewContent {
            new_grid,
            removed_rolls,
        } = remove_rolls_from_grid(grid);

        total_movable_paper = total_movable_paper + removed_rolls;

        grid = new_grid;

        if removed_rolls == 0 {
            break;
        }
    }

    println!("total_movable_paper {}", total_movable_paper)
}

struct Grid {
    width: i32,
    height: i32,
    content: HashMap<String, String>,
}

struct NewContent {
    new_grid: Grid,
    removed_rolls: i32,
}

fn remove_rolls_from_grid(
    Grid {
        height,
        width,
        mut content,
    }: Grid,
) -> NewContent {
    let mut total_movable_paper: i32 = 0;

    let mut keys = HashSet::new();

    for row in 0..height {
        for col in 0..width {
            if is_movable_paper(&content, row, col) {
                keys.insert(get_key(row, col));
                // println!("Movable paper at {} {}", row, col);
                total_movable_paper += 1;
            }
        }
    }

    for key in &keys {
        content.insert(key.clone(), String::from("."));
    }

    return NewContent {
        removed_rolls: total_movable_paper,
        new_grid: Grid {
            content,
            width,
            height,
        },
    };
}

fn parse_to_grid(input: &str) -> Grid {
    let mut grid_content: HashMap<String, String> = HashMap::new();
    let mut height: i32 = 0;

    let rows: Vec<&str> = input.lines().collect();
    let width = rows.len() as i32;

    for (row_index, line) in input.lines().enumerate() {
        let cols: Vec<char> = line.chars().collect();
        height = cols.len() as i32;

        for (col_index, content) in line.chars().enumerate() {
            grid_content.insert(
                get_key(row_index as i32, col_index as i32),
                content.to_string(),
            );
        }
    }

    return Grid {
        width,
        height,
        content: grid_content,
    };
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_movable_paper(grid: &HashMap<String, String>, row: i32, col: i32) -> bool {
    let mut adjacent_paper = 0;

    let cell_content = grid.get(&get_key(row, col));

    if !is_paper(cell_content) {
        return false;
    }

    for (dr, dc) in DIRECTIONS {
        let new_row = row + dr;
        let new_col = col + dc;
        let content = grid.get(&get_key(new_row, new_col));

        if is_paper(content) {
            adjacent_paper += 1;
        }
    }

    // println!("adjacent_paper {}", adjacent_paper);

    return adjacent_paper < 4;
}

fn is_paper(cell_content: Option<&String>) -> bool {
    return cell_content == Some(&String::from("@"));
}

fn get_key(row: i32, col: i32) -> String {
    return row.to_string() + "," + &col.to_string();
}
