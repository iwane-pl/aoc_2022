#[derive(Clone)]
pub struct Monkey {
    pub items: Vec<u128>,
    pub worry_op: (char, u128),
    pub divisor: u128,
    pub next_true: usize,
    pub next_false: usize,
    pub inspected: u32,
}

impl Monkey {
    pub fn inspect_items(&mut self, manage_worry: bool) -> Vec<(u128, usize)> {
        let mut throws = vec![];
        for item in self.items.drain(..) {
            self.inspected += 1;
            let mut worry_level = item;
            match self.worry_op.0 {
                '*' => worry_level *= self.worry_op.1,
                '+' => worry_level += self.worry_op.1,
                '^' => worry_level *= worry_level,
                op => {
                    panic!("invalid operation: {op}")
                }
            };

            if manage_worry {
                worry_level /= 3;
            }

            if worry_level % self.divisor == 0 {
                // println!("{} -> {}", worry_level, self.next_true);
                throws.push((worry_level, self.next_true));
            } else {
                // println!("{} -> {}", worry_level, self.next_false);
                throws.push((worry_level, self.next_false));
            }
        }

        throws
    }

    pub fn catch_item(&mut self, item: u128) {
        self.items.push(item);
    }

    pub fn get_inspections(&self) -> u32 {
        self.inspected
    }

    pub fn print_items(&self) {
        println!("{:?}", self.items);
    }
}

pub fn monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![92, 73, 86, 83, 65, 51, 55, 93],
            worry_op: ('*', 5),
            divisor: 11,
            next_true: 3,
            next_false: 4,
            inspected: 0,
        },
        Monkey {
            items: vec![99, 67, 62, 61, 59, 98],
            worry_op: ('^', 2),
            divisor: 2,
            next_true: 6,
            next_false: 7,
            inspected: 0,
        },
        Monkey {
            items: vec![81, 89, 56, 61, 99],
            worry_op: ('*', 7),
            divisor: 5,
            next_true: 1,
            next_false: 5,
            inspected: 0,
        },
        Monkey {
            items: vec![97, 74, 68],
            worry_op: ('+', 1),
            divisor: 17,
            next_true: 2,
            next_false: 5,
            inspected: 0,
        },
        Monkey {
            items: vec![78, 73],
            worry_op: ('+', 3),
            divisor: 19,
            next_true: 2,
            next_false: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![50],
            worry_op: ('+', 5),
            divisor: 7,
            next_true: 1,
            next_false: 6,
            inspected: 0,
        },
        Monkey {
            items: vec![95, 88, 53, 75],
            worry_op: ('+', 8),
            divisor: 3,
            next_true: 0,
            next_false: 7,
            inspected: 0,
        },
        Monkey {
            items: vec![50, 77, 98, 85, 94, 56, 89],
            worry_op: ('+', 2),
            divisor: 13,
            next_true: 4,
            next_false: 0,
            inspected: 0,
        },
    ]
}

pub fn monkeys_test() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            worry_op: ('*', 19),
            divisor: 23,
            next_true: 2,
            next_false: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            worry_op: ('+', 6),
            divisor: 19,
            next_true: 2,
            next_false: 0,
            inspected: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            worry_op: ('^', 2),
            divisor: 13,
            next_true: 1,
            next_false: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![74],
            worry_op: ('+', 3),
            divisor: 17,
            next_true: 0,
            next_false: 1,
            inspected: 0,
        },
    ]
}

pub fn day11_p1(_contents: &String, test: bool) -> u32 {
    let mut barrel: Vec<Monkey>;
    if test {
        barrel = monkeys_test();
    } else {
        barrel = monkeys();
    }

    for _ in 0..20 {
        for monkey_idx in 0..barrel.len() {
            // println!("Monkey {monkey_idx}");
            // barrel[monkey_idx].print_items();
            // println!();

            let throws = barrel[monkey_idx].inspect_items(true);

            for (item, next) in throws {
                barrel[next].catch_item(item);
            }
        }
    }

    let mut v = barrel
        .iter()
        .map(|m| m.get_inspections())
        .collect::<Vec<_>>();
    v.sort();
    v.iter().rev().take(2).fold(1, |acc, x| acc * x)
}

pub fn day11_p2(contents: &String, test: bool) -> u128 {
    let mut barrel: Vec<Monkey>;
    if test {
        barrel = monkeys_test();
    } else {
        barrel = monkeys();
    }

    for _ in 0..10000 {
        for monkey_idx in 0..barrel.len() {
            // println!("Monkey {monkey_idx}");
            // barrel[monkey_idx].print_items();
            // println!();

            let throws = barrel[monkey_idx].inspect_items(false);

            for (item, next) in throws {
                barrel[next].catch_item(item);
            }
        }
    }

    let mut v = barrel
        .iter()
        .map(|m| m.get_inspections())
        .collect::<Vec<_>>();
    v.sort();
    v.iter().rev().take(2).fold(1, |acc, x| acc * (*x as u128))
}
