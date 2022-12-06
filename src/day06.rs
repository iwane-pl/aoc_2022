use std::collections::HashSet;

pub fn day06_p1(contents: &String, distinct_chars: usize) -> u32 {
    let mut result = 0;

    let chars = contents.chars().collect::<Vec<char>>();

    for (idx, window) in chars.windows(distinct_chars).enumerate() {
        let sop = window.iter().collect::<HashSet<&char>>();

        if sop.len() == window.len() {
            result = (idx + window.len()) as u32;
            break;
        }
    }

    result
}
