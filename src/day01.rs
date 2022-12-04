use crate::parse_line;

pub fn day01_p1(contents: &String) -> u32 {
    let mut result: Vec<u32> = vec![];
    let mut calories: u32 = 0;

    for (index, line) in contents.lines().enumerate() {
        if line.is_empty() {
            result.push(calories);
            calories = 0;
        } else {
            let item: u32 = parse_line(line, index);
            calories += item;
        }
    }
    // push the last elf
    result.push(calories);

    result.sort();
    let max_cal: u32 = result.last().copied().unwrap();
    max_cal
}

pub fn day01_p2(contents: &String) -> u32 {
    let mut result: Vec<u32> = vec![];
    let mut calories: u32 = 0;

    for (index, line) in contents.lines().enumerate() {
        if line.is_empty() {
            result.push(calories);
            calories = 0;
        } else {
            let item: u32 = parse_line(&line, index);
            calories += item;
        }
    }
    result.push(calories);

    result.sort();
    let max = result.split_off(result.len() - 3);
    max.iter().sum()
}
