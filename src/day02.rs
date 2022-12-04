use std::collections::HashMap;

pub fn day02_p1(contents: &String) -> u32 {
    let mut points: HashMap<&str, u32> = HashMap::new();
    points.insert("A", 1); // Rock
    points.insert("B", 2); // Paper
    points.insert("C", 3); // Scissors

    let mut guide: HashMap<&str, &str> = HashMap::new();
    guide.insert("X", "A"); // Rock
    guide.insert("Y", "B"); // Paper
    guide.insert("Z", "C"); // Scissors

    let mut result: u32 = 0;

    for line in contents.lines() {
        let game_result: Vec<&str> = line.split(" ").collect();

        let oppo: &str = game_result[0];
        let my: &str = guide.get(game_result[1]).unwrap();

        result += points.get(my).unwrap();

        if oppo == my {
            result += 3; //tie
        } else {
            match (oppo, my) {
                ("A", "B") => {
                    result += 6;
                }
                ("A", "C") => {
                    result += 0;
                }
                ("B", "A") => {
                    result += 0;
                }
                ("B", "C") => {
                    result += 6;
                }
                ("C", "A") => {
                    result += 6;
                }
                ("C", "B") => {
                    result += 0;
                }
                (&_, _) => {
                    result += 0;
                }
            }
        }
    }

    result
}

pub fn day02_p2(contents: &String) -> u32 {
    let mut points: HashMap<&str, u32> = HashMap::new();
    points.insert("A", 1); // Rock
    points.insert("B", 2); // Paper
    points.insert("C", 3); // Scissors
    points.insert("X", 0); // Rock
    points.insert("Y", 3); // Paper
    points.insert("Z", 6); // Scissors

    let mut result: u32 = 0;

    for line in contents.lines() {
        let game_result: Vec<&str> = line.split(" ").collect();

        let oppo: &str = game_result[0];
        let exp_result: &str = game_result[1];

        result += points.get(exp_result).unwrap();

        let my: &str = match (oppo, exp_result) {
            ("A", "X") => "C",
            ("A", "Y") => "A",
            ("A", "Z") => "B",
            ("B", "X") => "A",
            ("B", "Y") => "B",
            ("B", "Z") => "C",
            ("C", "X") => "B",
            ("C", "Y") => "C",
            ("C", "Z") => "A",
            (&_, _) => "",
        };

        result += points.get(my).unwrap();
    }

    result
}
