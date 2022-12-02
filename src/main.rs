mod loader;
mod day01;

#[cfg(test)]
mod tests;

use loader::*;
use day01::*;

fn main() {
    println!("--- Advent of Code 2022 ---\n");

    let contents = load(r"../../../inputs/day01.txt");

    let calories = day01_p1(&contents);
    println!("Part 1: {calories}");
    let calories = day01_p2(&contents);
    println!("Part 2: {calories}");
    
}
