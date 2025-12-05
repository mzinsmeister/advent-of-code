use std::collections::HashSet;
use aoc_2025::*;

const DAY: u8 = 4;

const PAPER_ROLL: char = '@';

fn count_neighbors(input: &[Vec<char>], pos: (usize, usize)) -> usize {
    let mut neighboring = 0;
    let (x, y) = pos;
    if y > 0 {
        if x > 0 && input[y - 1][x - 1] == PAPER_ROLL {
            neighboring += 1;
        }
        if input[y - 1][x] == PAPER_ROLL {
            neighboring += 1;
        }
        if x+1 < input[0].len() && input[y - 1][x + 1] == PAPER_ROLL {
            neighboring += 1;
        }
    }
    if y+1 < input.len() {
        if x > 0 && input[y + 1][x - 1] == PAPER_ROLL {
            neighboring += 1;
        }
        if input[y + 1][x] == PAPER_ROLL {
            neighboring += 1;
        }
        if x+1 < input[0].len() && input[y + 1][x + 1] == PAPER_ROLL {
            neighboring += 1;
        }
    }
    if x > 0 && input[y][x - 1] == PAPER_ROLL {
        neighboring += 1;
    }
    if x+1 < input[0].len() && input[y][x + 1] == PAPER_ROLL {
        neighboring += 1;
    }
    neighboring
}

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let input: Vec<Vec<char>> = raw_input.lines().map(|l| l.chars().collect()).collect();

    let mut result1 = 0;

    let mut removable_positions = HashSet::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != PAPER_ROLL {
                continue;
            }
            let neighboring = count_neighbors(&input, (x, y));
            if neighboring < 4 {
                result1 += 1;
                removable_positions.insert((x, y));
            }
        }
    }

    println!("Result 1: {}", result1);

    let mut result2 = 0;
    let mut current_input = input.clone();

    while !removable_positions.is_empty() {
        for (x, y) in removable_positions.iter() {
            current_input[*y][*x] = '.';
            result2 += 1;
        }
        removable_positions.clear();
        for (y, line) in current_input.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c != PAPER_ROLL {
                    continue;
                }
                let neighboring = count_neighbors(&current_input, (x, y));
                if neighboring < 4 {
                    removable_positions.insert((x, y));
                }
            }
        }
    }


    println!("Result 2: {}", result2);

}