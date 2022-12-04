mod day01;
mod day02;
mod day03;
mod day04;
mod loader;

#[cfg(test)]
mod tests;

use day01::*;
use day02::*;
use day03::*;
use day04::*;
use loader::*;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.contains(&"all".to_string()) {
        for d in 1..=25 {
            args.push(format!("day{:02}", d).to_string());
        }
    }

    println!("--- Advent of Code 2022 ---");

    if args.contains(&"day01".to_string()) {
        println!("\n-- Day 1 --");

        let contents = load(r"../../../inputs/day01.txt");

        let calories = day01_p1(&contents);
        println!("Part 1: {calories}");
        let calories = day01_p2(&contents);
        println!("Part 2: {calories}");
    }

    if args.contains(&"day02".to_string()) {
        println!("\n-- Day 2 --");

        let contents = load(r"../../../inputs/day02.txt");

        let score = day02_p1(&contents);
        println!("Score 1: {score}");
        let score = day02_p2(&contents);
        println!("Score 2: {score}");
    }

    if args.contains(&"day03".to_string()) {
        println!("\n-- Day 3 --");

        let contents = load(r"../../../inputs/day03.txt");

        let score = day03_p1(&contents);
        println!("Score 1: {score}");
        let score = day03_p2(&contents);
        println!("Score 2: {score}");
    }

    if args.contains(&"day04".to_string()) {
        println!("\n-- Day 4 --");

        let contents = load(r"../../../inputs/day04.txt");

        let score = day04_p1(&contents);
        println!("Score 1: {score}");
        let score = day04_p2(&contents);
        println!("Score 2: {score}");
    }
}
