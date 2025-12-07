use std::collections::{HashMap, HashSet};

use aoc_2025::*;
use itertools::Itertools;

const DAY: u8 = 7;

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let input: Vec<Vec<char>> = raw_input.lines()
        .map(|l| l.chars().collect())
        .collect();

    let start_pos = input[0].iter().position(|c| *c == 'S').unwrap();
    let splitters: Vec<Vec<usize>> = input[1..].iter()
        .map(|l| l.iter().positions(|c| *c == '^').collect())
        .collect();

    let mut result1 = 0;

    let mut beams = HashSet::from([start_pos]);

    for line in splitters.iter() {
        for splitter in line {
            if beams.contains(&splitter) {
                result1 += 1;
                beams.remove(&splitter);
                beams.insert(splitter - 1);
                beams.insert(splitter + 1);
            }
        }
    }

    println!("Result 1: {}", result1);

    // All we need to do for part 2 is replace the set semantics of beams with multiset sematics
    // This then works basically the same as using memoization, just simpler

    let mut beams = HashMap::from([(start_pos, 1)]);

    for line in splitters {
        for splitter in line {
            if let Some(count) = beams.remove(&splitter) {
                *beams.entry(splitter - 1).or_default() += count;
                *beams.entry(splitter + 1).or_default() += count;
            }
        }
    }

    let result2: usize = beams.iter().map(|(_, c)| c).sum();

    println!("Result 2: {}", result2);

}