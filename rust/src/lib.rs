use aoc::utils::cli::{read_data, DataType, Part};

pub fn solve(r#type: DataType, part: Part, day: &str) {
    let contents = read_data(r#type, day);

    let val = match part {
        Part::One => part_one(&contents),
        Part::Two => part_two(&contents),
    };

    println!("Part {part} solution: {val:?}");
}

pub fn part_one(contents: &str) -> String {
    let GROUPS: Vec<&str> = contents.split('\n').collect();

    format!("")
}

pub fn part_two(contents: &str) -> String {
    format!("")
}
