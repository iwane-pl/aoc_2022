mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod loader;

#[cfg(test)]
mod tests;

use day01::*;
use day02::*;
use day03::*;
use day04::*;
use day05::*;
use day06::*;
use day07::*;
use day08::*;
use day09::*;
use day10::*;
use day11::*;
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

    if args.contains(&"day05".to_string()) {
        println!("\n-- Day 5 --");

        let contents = load(r"../../../inputs/day05.txt");

        let score = day05_p1(&contents, false);
        println!("Part 1: {score}");
        let score = day05_p1(&contents, true);
        println!("Part 2: {score}");
    }

    if args.contains(&"day06".to_string()) {
        println!("\n-- Day 6 --");

        let contents = load(r"../../../inputs/day06.txt");

        let score = day06_p1(&contents, 4);
        println!("Part 1: {score}");
        let score = day06_p1(&contents, 14);
        println!("Part 2: {score}");
    }

    if args.contains(&"day07".to_string()) {
        println!("\n-- Day 7 --");

        let contents = load(r"../../../inputs/day07.txt");

        let score = day07_p1(&contents);
        println!("Part 1: {score}");
        let score = day07_p2(&contents);
        println!("Part 2: {score}");
    }

    if args.contains(&"day08".to_string()) {
        println!("\n-- Day 08 --");

        let contents = load(r"../../../inputs/day08.txt");

        let score = day08_p1(&contents);
        println!("Part 1: {score}");
        let score = day08_p2(&contents);
        println!("Part 2: {score}");
    }

    if args.contains(&"day09".to_string()) {
        println!("\n-- Day 09 --");

        let contents = load(r"../../../inputs/day09.txt");

        let score = day09_p1(&contents);
        println!("Part 1: {score}");
        let score = day09_p2(&contents);
        println!("Part 2: {score}");
    }

    if args.contains(&"day10".to_string()) {
        println!("\n-- Day 10 --");

        let contents = load(r"../../../inputs/day10.txt");

        let score = day10_p1(&contents);
        println!("Part 1: {score}");
        let score = day10_p2(&contents);
        println!("Part 2: {score}");
    }

    if args.contains(&"day11".to_string()) {
        println!("\n-- Day 11 --");

        let contents = load(r"../../../inputs/day11.txt");

        let score = day11_p1(&contents, false);
        println!("Part 1: {score}");
        let score = day11_p2(&contents, false);
        println!("Part 2: {score}");
    }
}
