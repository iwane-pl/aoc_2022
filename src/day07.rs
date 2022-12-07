use std::collections::BTreeMap;
use std::str;

#[derive(Debug)]
pub struct FsEntry {
    pub etype: char,
    pub size: u32,
    pub path: String,
}

pub fn day07_p1(contents: &String) -> u32 {
    let mut result = 0;

    let mut filesystem: BTreeMap<(&str, char), FsEntry> = BTreeMap::new();
    let mut pwd: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.starts_with("$") {
            let cmd = line
                .trim_start_matches("$ ")
                .split_ascii_whitespace()
                .collect::<Vec<_>>();

            match cmd[0] {
                "cd" => {
                    if cmd[1] != ".." {
                        filesystem.insert(
                            (cmd[1], 'd'),
                            FsEntry {
                                etype: 'd',
                                size: 0,
                                path: pwd.join("/"),
                            },
                        );
                    }
                    match cmd[1] {
                        "/" => pwd = vec!["/"],
                        ".." => {
                            let _ = pwd.pop();
                        }
                        target => pwd.push(target),
                    };
                }
                "ls" => {}
                _ => {
                    panic!("Invalid command: {cmd:?}")
                }
            }
        } else {
            // files
            let entry = line.split_ascii_whitespace().collect::<Vec<_>>();

            match entry[0] {
                "dir" => {
                    filesystem.insert(
                        (entry[1], 'd'),
                        FsEntry {
                            etype: 'd',
                            size: 0,
                            path: pwd.join("/"),
                        },
                    );
                }
                size => {
                    let size = size.parse::<u32>().unwrap();
                    filesystem.insert(
                        (entry[1], 'f'),
                        FsEntry {
                            etype: 'f',
                            size: size,
                            path: pwd.join("/"),
                        },
                    );
                    for parent in pwd.clone().into_iter().rev() {
                        filesystem
                            .entry((parent, 'd'))
                            .and_modify(|e| e.size += size);
                    }
                }
            }
        }
    }

    for ((name, etype), entry) in filesystem.iter() {
        if entry.etype == 'd' && entry.size <= 100_000 {
            println!("found {name}:{entry:?}");
            result += entry.size;
        }
    }

    result
}

pub fn day07_p2(contents: &String) -> u32 {
    0
}
