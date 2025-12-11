use std::collections::HashMap;

use aoc_2025::*;

const DAY: u8 = 11;

fn dfs(input: &Vec<Vec<usize>>, current: usize, end: usize) -> usize {
    if current == end {
        return 1;
    }
    input[current].iter().map(|&c| dfs(input, c, end)).sum()
}

fn dfs2(memo_table: &mut HashMap<(usize, Vec<bool>), usize>, input: &Vec<Vec<usize>>, current: usize, end: usize, to_visit: &[usize], visited: &Vec<bool>) -> usize {
    if visited[current] {
        return 0;
    }
    let key2 = visited.iter().enumerate().filter(|(i, _)| to_visit.contains(i)).map(|(_, v)| *v).collect();
    if let Some(r) = memo_table.get(&(current, key2)) {
        return *r;
    }
    if current == end && to_visit.iter().all(|&e| visited[e]) {
        return 1;
    }
    let new_visited: Vec<bool> = visited.iter().enumerate().map(|(i, v)| *v || i == current).collect();
    let result = input[current].iter().map(|&c| dfs2(memo_table, input, c, end, to_visit, &new_visited)).sum();
    memo_table.insert((current, visited.iter().enumerate().filter(|(i, _)| to_visit.contains(i)).map(|(_, v)| *v).collect()), result);
    result
}

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let input_str: Vec<(&str, Vec<&str>)> = raw_input.lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(l, r)| (l, r.split(" ").collect()))
        .collect();

    let mut mapping = HashMap::new();
    let mut input = vec![];
    for (l, r) in input_str {
        let next_id = mapping.len();
        let l_id = *mapping.entry(l).or_insert(next_id);
        let mut r_ids = vec![];
        for r_elem in r {
            let next_id = mapping.len();
            let id = *mapping.entry(r_elem).or_insert(next_id);
            r_ids.push(id);
        }
        if input.len() < mapping.len() {
            input.resize(mapping.len(), vec![]);
        }
        input[l_id] = r_ids;
    }



    //let result1 = dfs(&input, mapping[&"you"], mapping[&"out"]);

    //println!("Result 1: {}", result1);

    let to_visit = vec![mapping[&"dac"], mapping[&"fft"]];
    let mut memo_table = HashMap::new();
    let result2 = dfs2(&mut memo_table, &input, mapping[&"svr"], mapping[&"out"], &to_visit, &vec![false; mapping.len()]);

    println!("Result 2: {}", result2);

}