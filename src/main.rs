mod loader;
mod day01;
mod day02;
mod day03;

#[cfg(test)]
mod tests;

use loader::*;
use day01::*;
use day02::*;
use day03::*;

fn main() {
    println!("--- Advent of Code 2022 ---");

    println!("\n-- Day 1 --");
    
    let contents = load(r"../../../inputs/day01.txt");
    
    let calories = day01_p1(&contents);
    println!("Part 1: {calories}");
    let calories = day01_p2(&contents);
    println!("Part 2: {calories}");
    
    println!("\n-- Day 2 --");
    
    let contents = load(r"../../../inputs/day02.txt");
    
    let score = day02_p1(&contents);
    println!("Score 1: {score}");
    let score = day02_p2(&contents);
    println!("Score 2: {score}");
    
    let contents = load(r"../../../inputs/day03.txt");
    
    let score = day03_p1(&contents);
    println!("Score 1: {score}");
    let score = day03_p2(&contents);
    println!("Score 2: {score}");

}
