mod loader;

#[cfg(test)]
mod tests;

use loader::*;

fn day01_p1(contents: &String) -> u32 {
    let mut result: Vec<u32> = vec![];
    let mut calories: u32 = 0;

    for (index, line) in contents.lines().enumerate() {
        if line.is_empty() {
            result.push(calories);
            calories = 0;
        }    
        else {
            let item: u32 = match line.parse() {
                Ok(num) => {num}
                Err(_) => {panic!("Invalid value at line {index}");}
            };
            calories += item;
        }
    }
    // push the last elf
    result.push(calories);

    result.sort();
    let max_cal: u32 = result.last().copied().unwrap();
    max_cal
}

fn day01_p2(contents: &String) -> u32 {
    let mut result: Vec<u32> = vec![];
    let mut calories: u32 = 0;

    for (index, line) in contents.lines().enumerate() {
        if line.is_empty() {
            result.push(calories);
            calories = 0;
        }    
        else {
            let item: u32 = match line.parse() {
                Ok(num) => {num}
                Err(_) => {panic!("Invalid value at line {index}");}
            };
            calories += item;
        }
    }
    result.push(calories);

    result.sort();
    let max = result.split_off(result.len() - 3);
    max.iter().sum()
}

fn main() {
    println!("--- Advent of Code 2022 ---\n");

    let contents = load(r"../../../inputs/day01.txt");

    let calories = day01_p1(&contents);
    println!("Part 1: {calories}");
    let calories = day01_p2(&contents);
    println!("Part 2: {calories}");
    
}
