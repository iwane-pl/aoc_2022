use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

fn is_visible(tree_map: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    // let mut visible = false;
    let size = tree_map[0].len();
    let tree = tree_map[x][y];

    // let visible_left = tree_map[x][..y].iter().all(|f| f < &tree);
    // let visible_right = tree_map[x][y..].iter().all(|f| f < &tree);

    let mut visible_top = true;
    let mut visible_bottom = true;
    for ty in 0..size {
        match ty.cmp(&y) {
            Ordering::Equal => {
                continue;
            }
            Ordering::Less => {
                visible_top &= tree > tree_map[x][ty];
            }
            Ordering::Greater => {
                visible_bottom &= tree > tree_map[x][ty];
            }
        }
    }
    let mut visible_left = true;
    let mut visible_right = true;
    for tx in 0..size {
        match tx.cmp(&x) {
            Ordering::Equal => {
                continue;
            }
            Ordering::Less => {
                visible_left &= tree > tree_map[tx][y];
            }
            Ordering::Greater => {
                visible_right &= tree > tree_map[tx][y];
            }
        }
    }

    // println!("Tree {x} {y} {visible_top:?} {visible_bottom:?} {visible_left:?} {visible_right:?}");

    visible_top || visible_bottom || visible_left || visible_right
}

pub fn scenic_score(tree_map: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    // let mut visible = false;
    let size = tree_map[0].len();
    let tree = tree_map[x][y];

    let mut score_top = 0;
    let mut score_bottom = 0;
    for ty in 0..size {
        match ty.cmp(&y) {
            Ordering::Equal => {
                continue;
            }
            Ordering::Less => {
                if score_top == 0 && tree <= tree_map[x][ty] {
                    score_top = y.abs_diff(ty);
                }
            }
            Ordering::Greater => {
                if score_bottom == 0 && tree <= tree_map[x][ty] {
                    score_bottom = y.abs_diff(ty);
                }
            }
        }
        if score_bottom > 0 && score_top > 0 {
            break;
        }
    }
    let mut score_left = 0;
    let mut score_right = 0;
    for tx in 0..size {
        match tx.cmp(&x) {
            Ordering::Equal => {
                continue;
            }
            Ordering::Less => {
                if score_left == 0 && tree <= tree_map[tx][y] {
                    score_left = x.abs_diff(tx);
                }
            }
            Ordering::Greater => {
                if score_right == 0 && tree <= tree_map[tx][y] {
                    score_right = x.abs_diff(tx);
                }
            }
        }
        if score_left > 0 && score_right > 0 {
            break;
        }
    }

    // println!("Tree {x} {y} {visible_top:?} {visible_bottom:?} {visible_left:?} {visible_right:?}");

    score_bottom + score_top + score_left + score_right
}

pub fn get_map(contents: &String) -> Vec<Vec<usize>> {
    let mut tree_map = vec![];

    for line in contents.lines() {
        let tree_line = line
            .chars()
            .map(|c| (c as usize - '0' as usize))
            .collect::<Vec<_>>();

        tree_map.push(tree_line);
    }

    tree_map
}

pub fn day08_p1(contents: &String) -> usize {
    let mut result = 0;

    let mut tree_map = get_map(contents);

    let size = tree_map[0].len() as usize;
    // outside tree count
    result += size + 2 * (size - 1) + (size - 2);
    println!("Outside tree count: {result}");

    for x in 1..(size - 1) {
        for y in 1..(size - 1) {
            let visible = is_visible(&tree_map, x, y);
            // println!("Tree {x} {y} is visible? {visible} ");
            if visible {
                result += 1;
            }
        }
    }

    result
}

pub fn day08_p2(contents: &String) -> usize {
    let mut result = 0;

    result
}
