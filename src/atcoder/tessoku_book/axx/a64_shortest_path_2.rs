// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bl

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

const INF: usize = std::usize::MAX;

fn run(n: usize, _m: usize, abc: Vec<(usize, usize, usize)>) -> Vec<isize> {
    let mut hash_map = HashMap::new();

    for (a, b, c) in abc {
        hash_map.entry(a).or_insert_with(|| Vec::new()).push((c, b));
        hash_map.entry(b).or_insert_with(|| Vec::new()).push((c, a));
    }

    let mut current = vec![INF; n+1];

    current[1] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, 1)));


    while let Some(Reverse((cur_cost, cur_i))) = priority_queue.pop() {
        if cur_cost > current[cur_i] {
            continue;
        }

        let Some(next) = hash_map.get(&cur_i) else {
            continue;
        };

        for (next_cost, next_i) in next {
            let new_cost = cur_cost + next_cost;

            if new_cost < current[*next_i] {
                current[*next_i] = new_cost;
                priority_queue.push(Reverse((new_cost, *next_i)));
            }
        }
    }

    current[1..].into_iter()
        .map(|c| {
            if *c == INF {
                -1
            } else {
                *c as isize
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, Vec<isize>);

    #[test]
    fn tessoku_a64() {
        let tests = [
            TestCase(6, 7, vec![(1, 2, 15), (1, 4, 20), (2, 3, 65), (2, 5, 4), (3, 6, 50), (4, 5, 30), (5, 6, 8)], vec![0, 15, 77, 20, 19, 27]),
            TestCase(15, 30, vec![ (10, 11, 23), (11, 13, 24), (5, 8, 22), (10, 15, 18), (12, 14, 15), (2, 10,  11), (4, 7, 15), (5, 7, 15), (7, 9, 8), (8, 12, 1), (5, 14, 1), (10, 14, 17), (10, 12, 11), (8, 10, 6), (7, 14, 28), (6, 9, 1), (1, 10, 19), (4, 5, 4), (9, 10, 21), (7, 10, 21), (4, 10, 29), (5, 10, 8), (4, 14, 8), (11, 12, 24), (10, 13, 13), (3, 10, 1), (5, 12, 24), (2, 15, 23), (6, 10, 18), (6, 15, 25)], vec![0, 30, 20, 31, 27, 37, 40, 25, 38, 19, 42, 26, 32, 28, 37]),
        ];

        for TestCase(n, m, abc, expected) in tests {
            assert_eq!(run(n, m, abc), expected);
        }
    }
}
