use aoc_2025::*;

const DAY: u8 = 3;

fn find_joltage(line: &[u32], active: u32) -> u64 {
    let mut digits: u64 = 0;
    let mut next_pos = 0;
    for digit in 0..active {
        let mut max = 0;
        let mut max_pos = 0;
        for (i, &val) in line.iter().enumerate().take(line.len() - (active - digit - 1) as usize).skip(next_pos) {
            if val > max {
                max = val;
                max_pos = i;
            }
        }
        digits *= 10;
        digits += max as u64;
        next_pos = max_pos + 1;
    }
    digits
}

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let input: Vec<Vec<u32>> = raw_input.split("\n")
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_joltage = 0;
    for line in input.iter() {
        total_joltage += find_joltage(line, 2);
    }

    let result1 = total_joltage;

    println!("Result 1: {}", result1);

    let mut total_joltage = 0;
    for line in input.iter() {
        total_joltage += find_joltage(line, 12);
    }

    let result2 = total_joltage;

    println!("Result 2: {}", result2);

}