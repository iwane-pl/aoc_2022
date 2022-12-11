struct CPU {
    reg_x: i32,
    clock: i32,
    crt: Vec<char>,
}

impl CPU {
    pub fn exec(&mut self, op: &str, arg: i32) -> i32 {
        let last_x = self.reg_x;
        match op {
            "addx" => {
                self.scan(2);
                // addx changes X value AFTER two cycles
                self.reg_x += arg;
            }
            "noop" => {
                self.scan(1);
            }
            _ => {
                panic!("invalid instruction: {op}");
            }
        }
        last_x
    }

    fn scan(&mut self, cycles: i32) {
        for _ in 0..cycles {
            let mask = self.clock % 40 - self.reg_x;
            let mut pixel = '.';
            if (-1..=1).contains(&mask) {
                pixel = '#';
            }
            self.crt.push(pixel);

            self.clock += 1;
        }
    }

    pub fn display(&self) {
        for l in self.crt.chunks(40) {
            let visible = l.iter().collect::<String>();
            println!("{visible}");
        }
    }

    pub fn cycle(&self) -> i32 {
        self.clock
    }
}

pub fn day10_p1(contents: &String) -> i32 {
    let mut result = 0;
    let mut cpu = CPU {
        reg_x: 1,
        clock: 0,
        crt: vec![],
    };
    let mut checkpoints = vec![];

    for line in contents.lines() {
        let cmd = line.split_ascii_whitespace().collect::<Vec<_>>();
        let op = cmd[0];
        let mut arg = 0;
        if cmd.len() > 1 {
            arg = cmd[1].parse().unwrap();
        }

        let last_x = cpu.exec(op, arg);

        let cur_cycle = cpu.cycle();
        let cycle_modulo = (cur_cycle + 20) % 40;
        if cur_cycle < 225 {
            match cycle_modulo {
                0 => {
                    println!("X during cycle {cur_cycle} = {last_x}");
                    result += last_x * cur_cycle;
                    checkpoints.push(cur_cycle);
                }
                1 => {
                    let adj_cycle = cur_cycle - 1;
                    if !checkpoints.contains(&adj_cycle) {
                        println!("X during cycle {adj_cycle} = {last_x}");
                        result += last_x * adj_cycle;
                        checkpoints.push(adj_cycle);
                    }
                }
                _ => {}
            }
        }
    }

    cpu.display();

    result
}

pub fn day10_p2(_contents: &String) -> usize {
    0
}
