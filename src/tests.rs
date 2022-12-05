use super::*;

#[test]
fn test_day01_p1() {
    let contents = load(r"..\..\..\..\tests\day01_p1.txt");

    let calories = day01_p1(&contents);

    assert_eq!(calories, 24000);
}

#[test]
fn test_day01_p2() {
    let contents = load(r"..\..\..\..\tests\day01_p1.txt");

    let calories = day01_p2(&contents);

    assert_eq!(calories, 45000);
}

#[test]
fn test_day02_p1() {
    let contents = load(r"..\..\..\..\tests\day02_p1.txt");

    let result = day02_p1(&contents);

    assert_eq!(result, 15);
}
#[test]
fn test_day02_p2() {
    let contents = load(r"..\..\..\..\tests\day02_p1.txt");

    let result = day02_p2(&contents);

    assert_eq!(result, 12);
}

#[test]
fn test_day03_p1() {
    let contents = load(r"..\..\..\..\tests\day03_p1.txt");

    let result = day03_p1(&contents);

    assert_eq!(result, 157);
}

#[test]
fn test_day03_p2() {
    let contents = load(r"..\..\..\..\tests\day03_p1.txt");

    let result = day03_p2(&contents);

    assert_eq!(result, 70);
}

#[test]
fn test_day04_p1() {
    let contents = load(r"..\..\..\..\tests\day04_p1.txt");

    let result = day04_p1(&contents);

    assert_eq!(result, 2);
}
#[test]
fn test_day04_p2() {
    let contents = load(r"..\..\..\..\tests\day04_p1.txt");

    let result = day04_p2(&contents);

    assert_eq!(result, 4);
}

#[test]
fn test_instruction_parse() {
    let contents = "move 1 from 2 to 1";

    let result: Instruction = contents.parse::<Instruction>().unwrap();

    assert_eq!(result.count, 1);
    assert_eq!(result.from, 1);
    assert_eq!(result.to, 0);
}

#[test]
fn test_day05_p1() {
    let contents = load(r"..\..\..\..\tests\day05_p1.txt");

    let result = day05_p1(&contents, false);

    assert_eq!(result, "CMZ".to_string());
}
#[test]
fn test_day05_p2() {
    let contents = load(r"..\..\..\..\tests\day05_p1.txt");

    let result = day05_p1(&contents, true);

    assert_eq!(result, "MCD".to_string());
}
