use std::collections::{HashMap, HashSet};

use aoc_2025::*;

const DAY: u8 = 2;

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let input: Vec<(u64, u64)> = raw_input.split(",")
        .map(|s| s.split_once("-")
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap())
        .collect();

    let mut invalid_sum: u64 = 0;

    for range in input.iter() {
        'id_loop:
        for n in range.0..=range.1 {
            let str = format!("{}", n);
            if str.len() % 2 != 0 {
                continue;
            }
            let repeat_len = str.len() / 2;
            if &str[0..repeat_len] == &str[repeat_len..2*repeat_len] {
                invalid_sum += n;
                continue 'id_loop;
            }
        }
    }


    let result1 = invalid_sum;

    println!("Result 1: {}", result1);

    let mut invalid_sum: u64 = 0;

    for range in input.iter() {
        'id_loop:
        for n in range.0..=range.1 {
            let str = format!("{}", n);
            'repeat_loop:
            for repeat_len in 1..=(str.len() / 2) {
                if str.len() % repeat_len != 0 {
                    continue;
                }
                for i in 1..str.len()/repeat_len {
                    if &str[i*repeat_len..(i+1)*repeat_len] != &str[0..repeat_len] {
                        continue 'repeat_loop;
                    }
                }
                invalid_sum += n;
                continue 'id_loop;
            }
        }
    }

    let result2 = invalid_sum;

    println!("Result 2: {}", result2);

}