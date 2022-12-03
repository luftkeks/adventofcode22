pub fn day1() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let mut number = 0;
    let mut numbers = vec![];
    for line in input.lines() {
        if !line.is_empty() {
            number += line.trim().parse::<i32>().unwrap();
        } else {
            numbers.push(number);
            number = 0
        }
    }

    numbers.sort();
    numbers.reverse();
    println!("Solution Day1a: {}", numbers.first().unwrap());
    println!("Solution Day1b: {}", numbers[0] + numbers[1] + numbers[2])
}
