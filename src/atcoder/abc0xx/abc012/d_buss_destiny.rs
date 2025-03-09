// https://atcoder.jp/contests/abc012/tasks/abc012_4

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::min;

fn dijkstra(n: usize, start: usize, hash_map: &HashMap<usize, Vec<(usize, usize)>>) -> usize {
    let mut dist = vec![std::usize::MAX; n+1];
    dist[start] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, start)));

    while let Some(Reverse((cur_cost, cur_i))) = priority_queue.pop() {
        for (next_cost, next_i) in hash_map.get(&cur_i).unwrap() {
            let new_cost = cur_cost + next_cost;

            if new_cost < dist[*next_i] {
                dist[*next_i] = new_cost;
                priority_queue.push(Reverse((new_cost, *next_i)));
            }
        }
    }

    dist.into_iter().filter(|n| *n != std::usize::MAX).max().unwrap()
}

fn run(n: usize, _m: usize, abt: Vec<(usize, usize, usize)>) -> usize {
    let mut ans = std::usize::MAX;

    let mut hash_map = HashMap::new();

    for (a, b, t) in abt {
        hash_map.entry(a).or_insert_with(|| Vec::new()).push((t, b));
        hash_map.entry(b).or_insert_with(|| Vec::new()).push((t, a));
    }

    for i in 1..=n {
        ans = min(ans, dijkstra(n, i, &hash_map));
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, vec![(1, 2, 10), (2, 3, 10)], 10),
            TestCase(5, 5, vec![(1, 2, 12), (2, 3, 14), (3, 4, 7), (4, 5, 9), (5, 1, 18)], 26),
            TestCase(4, 6, vec![(1, 2, 1), (2, 3, 1), (3, 4, 1), (4, 1, 1), (1, 3, 1), (4, 2, 1)], 1),
        ];

        for TestCase(n, m, abt, expected) in tests {
            assert_eq!(run(n, m, abt), expected);
        }
    }
}
