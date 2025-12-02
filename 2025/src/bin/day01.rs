use aoc_2025::*;

fn main() {
    let raw_input = read_input(1);

    let input = raw_input.as_str().split("\n")
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let dist: i32 = line[1..].parse().unwrap();
            match dir {
                'L' => -dist,
                'R' => dist,
                _ => panic!("Unknown direction"),
            }
        }).collect::<Vec<i32>>();


    let result1 = input.iter().scan(50, |acc, dir| {
            *acc += dir;
            *acc = acc.rem_euclid(100);
            Some(*acc)
        }).filter(|&d| d == 0)
        .count();

    println!("Result 1: {}", result1);

    let result2 = input.iter().scan(50, |acc, dir| {
        *acc += dir;
        let number_of_wraps = if *acc <= 0 && *acc != *dir {
            -*acc / 100 + 1
        } else {
            acc.abs() / 100
        };
        *acc = acc.rem_euclid(100);
        Some(number_of_wraps)
    }).sum::<i32>();

    println!("Result 2: {}", result2);

}