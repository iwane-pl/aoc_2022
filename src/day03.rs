use std::collections::HashSet;

fn priority(ch: &char) -> u32 {
    (ch.to_ascii_uppercase() as u32 - 'A' as u32 + 1) + (if ch.is_uppercase() { 26 } else { 0 })
}

pub fn day03_p1(contents: &String) -> u32 {
    let mut result: u32 = 0;
    for line in contents.lines() {
        let compartments = line.split_at(line.len() / 2);

        let mut comp1: HashSet<char> = HashSet::new();
        let mut comp2: HashSet<char> = HashSet::new();

        comp1.extend(compartments.0.chars());
        comp2.extend(compartments.1.chars());

        let common = comp1.intersection(&comp2).collect::<Vec<_>>();

        result += priority(&common[0]);
    }

    result
}

pub fn day03_p2(contents: &String) -> u32 {
    let mut result: u32 = 0;
    for group in contents.lines().collect::<Vec<_>>().chunks(3) {
        let mut comp1: HashSet<char> = HashSet::new();
        let mut comp2: HashSet<char> = HashSet::new();
        let mut comp3: HashSet<char> = HashSet::new();

        comp1.extend(group[0].chars());
        comp2.extend(group[1].chars());
        comp3.extend(group[2].chars());

        let common12 = comp1.intersection(&comp2);

        for item in common12 {
            if comp3.contains(&item) {
                result += priority(&item);
            }
        }
    }

    result
}
