use std::collections::HashSet;

pub fn day6() {
    let input = std::fs::read_to_string("input6.txt").unwrap();

    let mut first_task_reached = false;
    for ii in 4..input.len() {
        if !first_task_reached {
            let char_set: HashSet<char> = HashSet::from_iter(input.chars().skip(ii - 3).take(4));
            if char_set.len() == 4 {
                println!("Day 6: First Task: {}", ii + 1);
                first_task_reached = true;
            }
        }
        if ii >= 13 {
            let char_set: HashSet<char> = HashSet::from_iter(input.chars().skip(ii - 13).take(14));
            if char_set.len() == 14 {
                println!("Day 6: Second Task: {}", ii + 1);
                return;
            }
        }
    }
}
