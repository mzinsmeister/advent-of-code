use std::collections::BinaryHeap;

use aoc_2025::*;
use itertools::Itertools;
use petgraph::unionfind::UnionFind;

const DAY: u8 = 8;

fn distance_euclid(p1: &(f64, f64, f64), p2: &(f64, f64, f64)) -> f64 {
    ((p1.0 - p2.0).powf(2.0) +  (p1.1 - p2.1).powf(2.0) + (p1.2 - p2.2).powf(2.0)).sqrt()
}

const EPS: f64 = 1e-10;

fn main() {
    //let raw_input = read_demo(DAY);
    let raw_input = read_input(DAY);
    let num_connections = 1000;

    let input: Vec<(f64, f64, f64)> = raw_input.lines()
        .map(|l| l.split(","))
        .map(|mut p| (p.next().unwrap().parse().unwrap(), p.next().unwrap().parse().unwrap(), p.next().unwrap().parse().unwrap()))
        .collect();

    let mut unionfind: UnionFind<usize> = UnionFind::new(input.len());

    #[derive(Copy, Clone)]
    struct BinaryHeapElem {
        p1: usize,
        p2: usize,
        dist: f64
    }

    impl PartialEq for BinaryHeapElem {
        fn eq(&self, other: &Self) -> bool {
            (self.dist - other.dist).abs() < EPS
        }
    }

    impl Eq for BinaryHeapElem {}

    impl PartialOrd for BinaryHeapElem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(other.dist.total_cmp(&self.dist))
        }
    }

    impl Ord for BinaryHeapElem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    let mut distance_matrix: Vec<Vec<f64>> = vec![vec![f64::MAX; input.len()]; input.len()];
    let mut priority_queue: BinaryHeap<BinaryHeapElem> = BinaryHeap::new();

    for (i, p1) in input.iter().enumerate() {
        for (j, p2) in input.iter().enumerate().skip(i) {
            let dist = distance_euclid(p1, p2);
            distance_matrix[i][j] = dist;
            distance_matrix[j][i] = dist;
            if i != j {
                priority_queue.push(BinaryHeapElem { p1: i, p2: j, dist})
            }
        }
    }

    let mut prioritiy_queue2 = priority_queue.clone();
    for _ in 0..num_connections {
        let elem = priority_queue.pop().unwrap();
        unionfind.union(elem.p1, elem.p2);
    }

    let result1 = unionfind.into_labeling().iter().sorted().dedup_with_count().map(|(c, _)| c).sorted_by(|c1, c2| c2.cmp(c1)).take(3).product::<usize>();

    println!("Result 1: {}", result1);

    let mut unionfind: UnionFind<usize> = UnionFind::new(input.len());


    let mut components = input.len();
    loop {
        let elem = prioritiy_queue2.pop().unwrap();
        if !unionfind.equiv(elem.p1, elem.p2) {
            unionfind.union(elem.p1, elem.p2);
            components -= 1;
            if components == 1 {
                println!("Result 2: {}", input[elem.p1].0 * input[elem.p2].0);
                return;
            }
        }
    }


}