#[derive(Debug, Hash, Eq, PartialEq)]
pub struct FsEntry {
    pub name: String,
    pub etype: char,
    pub size: u32,
    pub path: String,
}

fn parse_filesystem(contents: &String) -> Vec<FsEntry> {
    let mut filesystem: Vec<FsEntry> = vec![];
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
                        let ppath = pwd.join("/");
                        let result = filesystem
                            .iter()
                            .find(|e| e.name == cmd[1] && e.path == ppath);
                        match result {
                            Some(_) => {}
                            None => {
                                filesystem.push(FsEntry {
                                    name: cmd[1].to_string(),
                                    etype: 'd',
                                    size: 0,
                                    path: ppath,
                                });
                            }
                        }
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
                    filesystem.push(FsEntry {
                        name: entry[1].to_string(),
                        etype: 'd',
                        size: 0,
                        path: pwd.join("/"),
                    });
                }
                size => {
                    let size = size.parse::<u32>().unwrap();
                    filesystem.push(FsEntry {
                        name: entry[1].to_string(),
                        etype: 'f',
                        size: size,
                        path: pwd.join("/"),
                    });

                    // parent modification
                    let mut visited = vec![];
                    for parent_name in pwd.iter() {
                        for entry in filesystem.iter_mut() {
                            if entry.name == parent_name.to_string()
                                && entry.etype == 'd'
                                && entry.path == visited.join("/")
                            {
                                entry.size += size;
                            }
                        }
                        visited.push(*parent_name);
                    }
                }
            }
        }
    }

    filesystem
}

pub fn day07_p1(contents: &String) -> u32 {
    let mut result = 0;

    let filesystem: Vec<FsEntry> = parse_filesystem(contents);

    for entry in filesystem.iter() {
        if entry.etype == 'd' && entry.size <= 100_000 {
            // println!("found {entry:?}");
            result += entry.size;
        }
    }

    result
}

pub fn day07_p2(contents: &String) -> u32 {
    let drive_size = 70000000;
    let needed_space = 30000000;

    let mut filesystem: Vec<FsEntry> = parse_filesystem(contents);

    let root = filesystem
        .iter()
        .find(|e| e.name == "/".to_string())
        .unwrap();
    // dbg!(root);

    let free_space = drive_size - root.size;
    let req_space = needed_space - free_space;
    // dbg!(req_space);

    filesystem.retain(|e| e.etype == 'd' && e.size > req_space);
    filesystem.sort_by(|a, b| a.size.cmp(&b.size));

    filesystem[0].size
}
