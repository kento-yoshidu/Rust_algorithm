// https://atcoder.jp/contests/abc035/tasks/abc035_d

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

const INF: usize = std::usize::MAX;

fn dijkstra(n: usize, start: usize, hash_map: &HashMap<usize, Vec<(usize, usize)>>) -> Vec<usize> {
    let mut dist = vec![INF; n+1];
    dist[start] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, start)));

    while let Some(Reverse((cur_cost, cur_i))) = priority_queue.pop() {
        if cur_cost > dist[cur_i] {
            continue;
        }

        if let Some(next) = hash_map.get(&cur_i) {
            for &(next_cost, next_i) in next {
                let new_cost = next_cost + cur_cost;

                if new_cost < dist[next_i] {
                    dist[next_i] = new_cost;
                    priority_queue.push(Reverse((new_cost, next_i)));
                }
            }
        }
    }

    dist
}

fn run(n: usize, _m: usize, t: usize, a: Vec<usize>, abc: Vec<(usize, usize, usize)>) -> usize {
    let mut forward = HashMap::new();
    let mut backward = HashMap::new();

    for (a, b, c) in abc {
        forward.entry(a).or_insert_with(Vec::new).push((c, b));
        backward.entry(b).or_insert_with(Vec::new).push((c, a));
    }

    let forward = dijkstra(n, 1, &forward);
    let backward = dijkstra(n, 1, &backward);

    let mut ans = 0;

    for i in 1..=n {
        if forward[i] == INF || backward[i] == INF {
            continue;
        }

        // 往復時間がT秒未満なら
        if forward[i] + backward[i] < t {
            ans = ans.max((t - forward[i] - backward[i]) * a[i-1]);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, 5, vec![1, 3], vec![(1, 2, 2), (2, 1, 1)], 6),
            TestCase(2, 2, 3, vec![1, 3], vec![(1, 2, 2), (2, 1, 1)], 3),
            TestCase(8, 15, 120, vec![1, 2, 6, 16, 1, 3, 11, 9], vec![(1, 8, 1), (7, 3, 14), (8, 2, 13), (3, 5, 4), (5, 7, 5), (6, 4, 1), (6, 8, 17), (7, 8, 5), (1, 4, 2), (4, 7, 1), (6, 1, 3), (3, 1, 10), (2, 6, 5), (2, 4, 12), (5, 1, 30)], 1488),
        ];

        for TestCase(n, m, t, a, abc, expected) in tests {
            assert_eq!(run(n, m, t, a, abc), expected);
        }
    }
}
