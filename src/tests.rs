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
fn test_day05_instruction_parse() {
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

#[test]
fn test_day06_p1() {
    let inputs: [(&str, u32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    for (idx, (contents, exp_pos)) in inputs.iter().enumerate() {
        let result = day06_p1(&contents.to_string(), 4);

        assert_eq!(result, *exp_pos, "failure at message {idx}");
    }
}

#[test]
fn test_day06_p2() {
    let inputs: [(&str, u32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    for (idx, (contents, exp_pos)) in inputs.iter().enumerate() {
        let result = day06_p1(&contents.to_string(), 14);

        assert_eq!(result, *exp_pos, "failure at message {idx}");
    }
}

#[test]
fn test_day07_p1() {
    let contents = load(r"..\..\..\..\tests\day07_p1.txt");

    let result = day07_p1(&contents);

    assert_eq!(result, 95437);
    // assert_eq!(result, 0);
}

#[test]
fn test_day07_p2() {
    let contents = load(r"..\..\..\..\tests\day07_p1.txt");

    let result = day07_p2(&contents);

    assert_eq!(result, 24933642);
    // assert_eq!(result, 0);
}

#[test]
fn test_day08_p1() {
    let contents = load(r"..\..\..\..\tests\day08_p1.txt");

    let result = day08_p1(&contents);

    assert_eq!(result, 21);
}
#[test]
fn test_day08_p2() {
    let contents = load(r"..\..\..\..\tests\day08_p1.txt");

    let result = day08_p2(&contents);

    assert_eq!(result, 8);
}

#[test]
fn test_day08_score_1() {
    let contents = load(r"..\..\..\..\tests\day08_p1.txt");
    let map = get_map(&contents);
    let result = scenic_score(&map, 2, 1);

    assert_eq!(result, 4);
}

#[test]
fn test_day08_score_2() {
    let contents = load(r"..\..\..\..\tests\day08_p1.txt");
    let map = get_map(&contents);
    let result = scenic_score(&map, 2, 3);

    assert_eq!(result, 8);
}

#[test]
fn test_day09_p1() {
    let contents = load(r"..\..\..\..\tests\day09_p1.txt");

    let result = day09_p1(&contents);

    assert_eq!(result, 13);
}

#[test]
fn test_day09_p2_1() {
    let contents = load(r"..\..\..\..\tests\day09_p2_1.txt");

    let result = day09_p2(&contents);

    assert_eq!(result, 1);
}

#[test]
fn test_day09_p2_2() {
    let contents = load(r"..\..\..\..\tests\day09_p2_2.txt");

    let result = day09_p2(&contents);

    assert_eq!(result, 36);
}

#[test]
fn test_day10_p1() {
    let contents = load(r"..\..\..\..\tests\day10_p1.txt");

    let result = day10_p1(&contents);

    assert_eq!(result, 13140);
}
#[test]
fn test_day10_p2() {
    let contents = load(r"..\..\..\..\tests\day10_p1.txt");

    let result = day10_p2(&contents);

    assert_eq!(result, 8);
}

#[test]
fn test_day11_p1() {
    let contents = load(r"..\..\..\..\tests\day11_p1.txt");

    let result = day11_p1(&contents, true);

    assert_eq!(result, 10605);
}

#[test]
fn test_day11_p2() {
    let contents = load(r"..\..\..\..\tests\day11_p1.txt");

    let result = day11_p2(&contents, true);

    assert_eq!(result, 2713310158);
}
