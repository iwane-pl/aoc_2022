use std::ops::RangeInclusive;

fn parse_elf(elf: &str) -> RangeInclusive<u32> {
    // parses an elf assignment
    let r1: Vec<&str> = elf.split('-').collect();
    (r1[0].parse::<u32>().unwrap())..=(r1[1].parse::<u32>().unwrap())
}

pub fn day04_p1(contents: &String) -> u32 {
    let mut result = 0;

    for line in contents.lines() {
        let groups: Vec<&str> = line.split(",").collect();

        let r1 = parse_elf(groups[0]);
        let r2 = parse_elf(groups[1]);

        if (r1.contains(r2.start()) && r1.contains(r2.end()))
            || (r2.contains(r1.start()) && r2.contains(r1.end()))
        {
            result += 1;
        }
    }

    result
}

pub fn day04_p2(contents: &String) -> u32 {
    let mut result: u32 = 0;

    for line in contents.lines() {
        let groups: Vec<&str> = line.split(",").collect();

        let r1 = parse_elf(groups[0]);
        let r2 = parse_elf(groups[1]);

        if (r1.contains(r2.start()) || r1.contains(r2.end()))
            || (r2.contains(r1.start()) || r2.contains(r1.end()))
        {
            result += 1;
        }
    }
    result
}
