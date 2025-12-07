use aoc_2025::*;

const DAY: u8 = 6;

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let input = raw_input.lines()
        .map(|line| line.split_whitespace().collect())
        .collect::<Vec<Vec<&str>>>();

    let mut numbers = vec![vec![]; input[0].len()];
    for line in input.iter().take(input.len() - 1) {
        for (i, n) in line.iter().enumerate() {
            numbers[i].push(n.parse::<u64>().unwrap());
        }
    }
    let ops = input.last().unwrap();

    let mut result1 = 0;


    for (numbers, op) in numbers.iter().zip(ops) {
        result1 += match *op {
            "*" => numbers.iter().fold(1, |a, e| a*e),
            "+" => numbers.iter().fold(0, |a, e| a+e),
            _ => unreachable!("operator doesn't exist: {}", op)
        }
    }

    println!("Result 1: {}", result1);

    let num_len = input.len() - 1;

    let mut col_lens: Vec<usize> = vec![];

    for col in 0..input[0].len() {
        let mut lenmax = 0;
        for n in input.iter() {
            lenmax = n[col].len().max(lenmax);
        }
        col_lens.push(lenmax);
    }

    let mut input = vec![];
    for line in raw_input.lines().take(num_len) {
        let mut line_input = vec![];
        let mut start = 0;
        while start < line.len() {
            let len = col_lens[line_input.len()];
            line_input.push(&line[start..start+len]);
            start += len + 1;
        }
        input.push(line_input);
    }
        

    let mut numbers = vec![vec![]; input[0].len()];
    for line in input.iter().take(input.len()) {
        for (i, n) in line.iter().enumerate() {
            let mut digits: Vec<char> = n.chars().collect();
            digits.reverse();
            for (j, d) in digits.iter().enumerate() {
                if *d == ' ' {
                    if numbers[i].len() <= j {
                        numbers[i].push(0);
                    }
                    continue;
                }
                let d = d.to_digit(10).unwrap();
                if numbers[i].len() <= j {
                    numbers[i].push(d as u64);
                } else {
                    numbers[i][j] *= 10;
                    numbers[i][j] += d as u64;
                }
            }
        }
    }

    let mut result2 = 0;

    for (numbers, op) in numbers.iter().zip(ops) {
        result2 += match *op {
            "*" => numbers.iter().fold(1, |a, e| a*e),
            "+" => numbers.iter().fold(0, |a, e| a+e),
            _ => unreachable!("operator doesn't exist: {}", op)
        }
    }

    println!("Result 2: {}", result2);

}