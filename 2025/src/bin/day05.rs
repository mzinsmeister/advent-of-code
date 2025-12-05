use aoc_2025::*;

const DAY: u8 = 5;

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let (fresh_input, available_input) = raw_input.split_once("\n\n").unwrap();

    let fresh: Vec<(u64, u64)> = fresh_input.split("\n")
        .map(|l| l.split_once("-").unwrap())
        .map(|(l,r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let check_fresh = |id: u64| {
        fresh.iter().any(|&(start, end)| id >= start && id <= end)
    };

    let available: Vec<u64> = available_input.split("\n")
        .map(|l| l.parse().unwrap())
        .collect();


    let result1 = available.iter().filter(|id| check_fresh(**id)).count();

    println!("Result 1: {}", result1);

    let mut fresh2 = fresh.clone();
    fresh2.sort_by(|a, b| a.0.cmp(&b.0));

    let mut changed = true;
    while changed {
        let mut new_ranges: Vec<(u64, u64)> = vec![];
        'outer:
        for range in fresh2.iter() {
            // Check if it overlaps with any other range, if so merge
            let mut new_range = *range;
            for check_range in new_ranges.iter() {
                // Completely contained: discard
                if new_range.0 >= check_range.0 && new_range.1 <= check_range.1 {
                    continue 'outer;
                }
                if new_range.0 >= check_range.0 && new_range.0 <= check_range.1 {
                    new_range.0 = check_range.1 + 1;
                }
                if new_range.1 >= check_range.0 && new_range.1 <= check_range.0 {
                    new_range.1 = check_range.0 - 1;
                }
            }
            new_ranges.push(new_range);
        }
        changed = new_ranges != fresh2;
        fresh2 = new_ranges;
    }

    let result2 = fresh2.iter().map(|(start, end)| end-start + 1).sum::<u64>();

    println!("Result 2: {}", result2);

}