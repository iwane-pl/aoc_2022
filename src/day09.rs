pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn move_by(&mut self, dir: &str, dist: u32) {
        match dir {
            "R" => {
                self.x += dist;
            }
            "L" => {
                self.x -= dist;
            }
            "U" => {
                self.y += dist;
            }
            "D" => {
                self.y -= dist;
            }
            _ => {
                panic!("Invalid character: {dir}");
            }
        }
    }

    pub fn pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

pub struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        }
    }
}

pub fn day09_p1(contents: &String) -> usize {
    let mut result = 0;

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    for line in contents.lines() {
        let cmd = line.split_ascii_whitespace().collect::<Vec<_>>();
        head.move_by(cmd[0], cmd[1].parse::<u32>().unwrap());
    }

    result
}

pub fn day09_p2(contents: &String) -> usize {
    let mut result = 0;

    result
}
