use std::{num::ParseIntError, str::FromStr};

pub struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // nice syntax...
        let s = s
            .strip_prefix("move ")
            .and_then(|s| Some(s.replace(" from ", ",")))
            .and_then(|s| Some(s.replace(" to ", ",")))
            .unwrap();

        let v = s.split(",").collect::<Vec<&str>>();

        let count_fromstr = v[0].parse::<usize>()?;
        let from_fromstr = v[1].parse::<usize>()?;
        let to_fromstr = v[2].parse::<usize>()?;

        Ok(Instruction {
            count: count_fromstr,
            from: from_fromstr - 1,
            to: to_fromstr - 1,
        })
    }
}

fn parse_stack(stack: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];
    let mut indices = vec![];
    for (line_index, line) in stack.lines().rev().enumerate() {
        if line_index == 0 {
            // create stacks
            for (idx, ch) in line.char_indices() {
                if ch.is_digit(10) {
                    result.push(vec![]);
                    indices.push(idx);
                }
            }
        } else {
            for (stack, line_pos) in indices.iter().enumerate() {
                let tvec: Vec<char> = line.chars().collect();
                match tvec[*line_pos] {
                    token if token.is_ascii_alphabetic() => {
                        result[stack].push(token);
                    }
                    _ => {
                        continue;
                    }
                };
            }
        }
    }

    result
}

pub fn day05_p1(contents: &String, is_9001: bool) -> String {
    let mut result = String::new();

    // 2 newlines separate stacks from instructions
    let parts = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut pile = parse_stack(parts[0]);
    for line in parts[1].lines() {
        let op = line.parse::<Instruction>().unwrap();

        let source_stack = &mut pile[op.from as usize];
        let mut slice = source_stack.split_off(source_stack.len() - op.count);
        if !is_9001 {
            slice.reverse();
        }

        pile[op.to].extend(slice.iter());
    }

    for stack in pile.iter() {
        result.push(*stack.last().unwrap());
    }

    result
}
