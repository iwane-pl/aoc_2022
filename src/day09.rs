use std::collections::HashSet;

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub visited: HashSet<(i32, i32)>,
}

impl Point {
    pub fn new() -> Point {
        Point {
            x: 0,
            y: 0,
            visited: HashSet::from([(0, 0)]),
        }
    }

    pub fn move_by_dir(&mut self, dir: &str, dist: i32) {
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
                panic!("Invalid direction: {dir}");
            }
        }

        self.visited.insert(self.pos());
    }

    pub fn move_by_pos(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
        self.visited.insert(self.pos());
    }

    pub fn pos(&self) -> (i32, i32) {
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
            head: Point::new(),
            tail: Point::new(),
        }
    }

    pub fn move_head(&mut self, dir: &str, dist: i32) {
        for _ in 0..dist {
            self.head.move_by_dir(dir, 1);

            let dx = self.head.x - self.tail.x;
            let dy = self.head.y - self.tail.y;

            if dx.abs() >= 2 || dy.abs() >= 2 {
                self.tail.move_by_pos(dx.signum(), dy.signum());
            }
        }
    }

    pub fn get_tail_visits(&self) -> HashSet<(i32, i32)> {
        self.tail.visited.clone()
    }
}

pub fn day09_p1(contents: &String) -> usize {
    let mut result = 0;

    let mut rope = Rope::new();

    for line in contents.lines() {
        let cmd = line.split_ascii_whitespace().collect::<Vec<_>>();
        rope.move_head(cmd[0], cmd[1].parse::<i32>().unwrap());
    }

    rope.get_tail_visits().len()
}

pub fn day09_p2(contents: &String) -> usize {
    let mut result = 0;

    result
}
