// https://atcoder.jp/contests/abc051/tasks/abc051_d

use core::hash;
use std::collections::{BinaryHeap, HashMap};

const INF: usize = std::usize::MAX;

fn dijkstra(n: usize, start: usize, hash_map: &HashMap<usize, Vec<(usize, usize)>>) -> Vec<usize> {
    let mut dist = vec![INF; n+1];
    dist[start] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((0, start));

    while let Some((cur_cost, cur_i)) = priority_queue.pop() {
        let Some(next) = hash_map.get(&cur_i) else {
            continue;
        };

        for (next_cost, next_i) in next {
            let new_cost = cur_cost + next_cost;

            if new_cost < dist[*next_i] {
                dist[*next_i] = new_cost;
                priority_queue.push((new_cost, *next_i));
            }
        }
    }

    dist
}

fn run(n: usize, m: usize, abc: Vec<(usize, usize, usize)>) -> usize {
    let mut hash_map = HashMap::new();

    for &(a, b, c) in &abc {
        hash_map.entry(a).or_insert_with(|| Vec::new()).push((c, b));
        hash_map.entry(b).or_insert_with(|| Vec::new()).push((c, a));
    }

    let mut dist = vec![vec![INF; n + 1]; n + 1];

    for i in 1..=n {
        dist[i] = dijkstra(n, i, &hash_map);
    }

    let mut used = vec![false; m];

    for i in 1..=n {
        for j in 1..=n {
            if i == j || dist[i][j] == INF {
                continue;
            }

            for (k, &(a, b, c)) in abc.iter().enumerate() {
                if dist[i][a] + c + dist[b][j] == dist[i][j] || dist[i][b] + c + dist[a][j] == dist[i][j] {
                    used[k] = true;
                }
            }
        }
    }

    used.iter().filter(|&&u| !u).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec![(1, 2, 1), (1, 3, 1), (2, 3, 3)], 1),
            TestCase(3, 2, vec![(1, 2, 1), (2, 3, 1)], 0),
        ];

        for TestCase(n, m, abc, expected) in tests {
            assert_eq!(run(n, m, abc), expected);
        }
    }
}
