use std::collections::HashMap;

use aoc_2025::*;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

const DAY: u8 = 9;

fn area(a: &(u64, u64), b: &(u64, u64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let input: Vec<(u64, u64)> = raw_input.lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(a,b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let mut max = 0;

    for (i, p1) in input.iter().enumerate() {
        for p2 in input.iter().skip(i+1) {
            max = max.max(area(p1, p2))
        }
    }

    let result1 = max;

    println!("Result 1: {}", result1);

    let x_map: HashMap<u64, usize> = input.iter()
        .map(|(x, _)| x)
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect();
    let y_map: HashMap<u64, usize> = input.iter()
        .map(|(_, y)| y)
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, y)| (*y, i))
        .collect();

    let minified_input: Vec<(usize, usize)> = input.iter().map(|(x, y)| (x_map[x], y_map[y])).collect();

    let mut vert_lines = vec![];

    for i in 0..minified_input.len() {
        let p1 = minified_input[i];
        let p2 = minified_input[(i + 1) % minified_input.len()];
        if p1.0 == p2.0 {
            // Vertical line
            vert_lines.push((p1.0, (p1.1.min(p2.1), p1.1.max(p2.1))));
        }
    }

    let test_inclusion = |p: &(usize, usize)| {
        let intersecting = vert_lines.iter().filter(|&&(x, (start, end))| x <= p.0 && p.1 >= start && p.1 <= end);
        let mut count = 0;
        let mut start_end = 0;
        for &(_, (start, end)) in intersecting {
            count += 1;
            if p.1 == start || p.1 == end {
                start_end += 1;
            }
        }
        (count - start_end / 2) % 2 == 1
    };


    // It's a more brute force solution with shortcuts
    // It runs in roughly 1s with --release on my laptop. Surely there are faster solutions
    let result2 = minified_input.par_iter().enumerate().map(|(i, p1)| {
        let mut max = 0;
        'inner:
        for (j, p2) in minified_input.iter().enumerate().skip(i+1) {
            for xtest in p1.0.min(p2.0)..=p1.0.max(p2.0) {
                for ytest in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                    if !test_inclusion(&(xtest, ytest)) {
                        continue 'inner;
                    }
                }
            }
            let p1 = input[i];
            let p2 = input[j];
            max = max.max(area(&p1, &p2));
        }
        max
    }).max().unwrap();


    println!("Result 2: {}", result2);

}