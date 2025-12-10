use std::{cmp::min, collections::{BTreeMap, HashMap, HashSet, btree_map::IterMut}, hash::Hash, usize};

use aoc_2025::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use z3::{Config, Context, Optimize, SatResult, Solver, ast::Int};

const DAY: u8 = 10;

struct InputLine {
    config: u64,
    buttons: Vec<Vec<usize>>,
    buttons_bin: Vec<u64>,
    joltage: Vec<u16>
}

impl InputLine {
    fn parse(input: &str) -> Self {
        let (config_raw, rest) = input.split_once(" ").unwrap();
        let mut config = 0u64;
        for (i, c) in config_raw.chars().take(config_raw.len()-1).skip(1).enumerate() {
            config |= ((c == '#') as u64) << i;
        }
        let mut buttons_bin: Vec<u64> = vec![];
        let mut buttons: Vec<Vec<usize>> = vec![];
        let mut joltage: Vec<u16> = vec![];
        for elem in rest.split(" ") {
            if elem.starts_with("(") {
                let mut button = 0u64;
                let mut button_vec = vec![];
                for e in elem.strip_prefix("(").unwrap().strip_suffix(")").unwrap().split(",") {
                    let num = e.parse::<usize>().unwrap();
                    button_vec.push(num);
                    button |= 1 << (num);
                }
                buttons_bin.push(button);
                buttons.push(button_vec);
            } else {
                for e in elem.strip_prefix("{").unwrap().strip_suffix("}").unwrap().split(",") {
                    joltage.push(e.parse().unwrap())
                }
            }
        }
        Self {
            config,
            buttons,
            buttons_bin,
            joltage
        }
    }

    fn solve1(&self) -> usize {
        let mut dp_table: BTreeMap<usize, HashMap<u64, u64>> = BTreeMap::new();
        
        dp_table.insert(1, HashMap::new());
        for (i, b) in self.buttons_bin.iter().enumerate() {
            if *b == self.config {
                return 1;
            }
            dp_table.get_mut(&1).unwrap().insert(1 << i, *b);
        }

        while *dp_table.last_key_value().unwrap().0 <= self.buttons_bin.len() {
            let (cur_len, cur_tbl) = dp_table.last_key_value().unwrap();
            let new_len = cur_len + 1;
            let mut new_tbl: HashMap<u64, u64> = HashMap::new();
            
            for (&active, &result) in cur_tbl {
                for (&active1, &result1) in dp_table[&1].iter() {
                    if (active | active1).count_ones() as usize == new_len {
                        let new_active = active | active1;
                        let new_result = result ^ result1;
                        if new_result == self.config {
                            return new_len;
                        }
                        new_tbl.insert(new_active, new_result);
                    }
                }
            }
            dp_table.insert(new_len, new_tbl);
        }

        println!("Config: {:012b}", self.config);
        for level in 1..self.buttons.len() {
            println!("{}:", level);
            for (&key, &value) in &dp_table[&level] {
                println!("{:#012b}: {:#012b}", key as u16, value as u16);
            }
            println!("\n\n");
        }
        panic!("No solution found for line");
    }

    fn solve2(&self) -> usize {
        let solver = Optimize::new();

        // Variables
        let mut vars = vec![];
        for x in 0..self.buttons.len() {
            vars.push(Int::new_const(format!("b{}", x)));
            solver.assert(&vars.last().unwrap().ge(0))
        }


        for (i, j) in self.joltage.iter().enumerate() {
            let mut buttons: Vec<usize> = self.buttons.iter().enumerate().filter(|(_,b)| b.contains(&i)).map(|(bi,_)| bi).collect();
            let mut obj = vars[buttons[0]].clone();
            for b in buttons.iter().skip(1) {
                obj = obj + &vars[*b];
            }
            solver.assert(&obj.eq(&Int::from_u64(*j as u64)));
        }

        let mut objective = vars[0].clone();

        for v in vars.iter().skip(1) {
            objective = objective + v;
        }
        solver.minimize(&objective);

        //let smt2 = solver.to_string();
        //println!("{}", smt2);
        // Check satisfiability
        match solver.check(&[]) {
            SatResult::Sat => {
                let model = solver.get_model().unwrap();
                let mut solution: usize = 0;
                for v in vars {
                    solution += model.eval(&v, true).unwrap().as_u64().unwrap() as usize;
                }
                //println!("SAT");
                return solution;
            }
            SatResult::Unsat => {
                panic!("UNSAT");
            }
            SatResult::Unknown => {
                panic!("UNKNOWN");
            }
        }
    }
}

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);

    let input: Vec<InputLine> = raw_input.lines().map(|l| InputLine::parse(l)).collect();

    let result1 = input.iter().map(|i| i.solve1()).sum::<usize>();

    println!("Result 1: {}", result1);

    let result2 = input.iter().map(|i| i.solve2()).sum::<usize>();

    println!("Result 2: {}", result2);

}