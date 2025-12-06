use regex::Regex;

fn main() {
    let input =
        std::fs::read_to_string("src/bin/day06/input.txt").expect("Failed to read input.txt");

    let Data { rows, operations } = parse_data_part_1(&input);

    if rows.first().unwrap().len() != operations.len() {
        panic!("Rows have different lengths");
    }
    let mut total = 0;

    for (index, symbol) in operations.iter().enumerate() {
        let operation = symbol.as_str();

        match operation {
            "+" => {
                let result = rows.iter().map(|row| row[index]).sum::<i64>();
                println!("Result: {}", result);
                total += result;
            }
            "*" => {
                let result = rows.iter().map(|row| row[index]).product::<i64>();
                println!("Result: {}", result);
                total += result;
            }
            _ => panic!("Unknown operation: {}", operation),
        }
    }
    println!("Total: {}", total);

    do_part_2(&input);
}

struct Data {
    rows: Vec<Vec<i64>>,
    operations: Vec<String>,
}

fn parse_data_part_1(input: &str) -> Data {
    let mut lines = input.lines().peekable();

    let mut rows: Vec<Vec<i64>> = Vec::new();
    let mut operations: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        if lines.peek().is_none() {
            let symbols_regex = Regex::new(r"[\*\+]").unwrap();
            operations = symbols_regex
                .find_iter(line)
                .map(|m| m.as_str().to_string())
                .collect();
        } else {
            // Process normally
            let numbers_regex = Regex::new(r"\d+").unwrap();
            let row = numbers_regex
                .find_iter(line)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            rows.push(row);
        }
    }

    return Data { rows, operations };
}

fn do_part_2(input: &str) -> () {
    let mut lines = input.lines().peekable();

    let mut rows: Vec<&str> = Vec::new();
    let mut operations: &str = "";

    while let Some(line) = lines.next() {
        if lines.peek().is_none() {
            operations = line
        } else {
            rows.push(line);
        }
    }

    let row_length = operations.chars().count();

    let miss_sized_rows = rows
        .iter()
        .filter(|row| row.chars().count() != row_length)
        .count();

    if miss_sized_rows > 0 {
        panic!("Miss sized rows: {}", miss_sized_rows);
    }

    let mut sum: usize = 0;

    loop {
        // get the char in each row and operation
        let index = operations
            .char_indices()
            .position(|(index, operation_str_char)| {
                operation_str_char == ' '
                    && rows
                        .iter()
                        .all(|row| row.chars().nth(index).unwrap() == ' ')
            });

        if index.is_none() {
            println!("No index found");
            break;
        }

        println!("Found index {:?}", index.unwrap());

        // this is the position of the col break - take everything before it
        let index = index.unwrap();

        let operation = &operations[..index];

        let mut numbers: Vec<usize> = Vec::new();

        for i in 0..index {
            let mut number_raw = String::new();
            for row in &rows {
                let char = row.chars().nth(i).unwrap();
                number_raw.push(char);
            }

            println!("Number raw: {}, index: {}", number_raw, i);
            numbers.push(number_raw.trim().parse::<usize>().unwrap());
        }

        let symbols_regex = Regex::new(r"[\*\+]").unwrap();
        let matched_operation = symbols_regex.find(operation).unwrap();

        match matched_operation.as_str() {
            "+" => {
                let result = numbers.iter().sum::<usize>();
                sum += result;
            }
            "*" => {
                let result = numbers.iter().product::<usize>();
                sum += result;
            }
            _ => panic!("Unknown operation: {}", matched_operation.as_str()),
        }

        // Now reset the rows and operations
        rows = rows
            .iter()
            .map(|row| &row[index + 1..])
            .collect::<Vec<&str>>();
        operations = &operations[index + 1..];
    }

    println!("Sum: {}", sum);
}
