// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fl

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

fn dijkstra(start: usize, n: usize, map: &HashMap<usize, Vec<(usize, usize)>>) -> Vec<usize> {
    let mut dist = vec![usize::MAX; n + 1];
    dist[start] = 0;

    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, start)));

    while let Some(Reverse((cur_cost, cur))) = pq.pop() {
        if cur_cost > dist[cur] {
            continue;
        }

        if let Some(nexts) = map.get(&cur) {
            for &(cost, to) in nexts {
                let new_cost = cur_cost + cost;
                if new_cost < dist[to] {
                    dist[to] = new_cost;
                    pq.push(Reverse((new_cost, to)));
                }
            }
        }
    }

    dist
}

fn run(n: usize, _m: usize, abc: Vec<(usize, usize, usize)>) -> usize {
    let mut map = HashMap::new();

    for (a, b, c) in abc {
        map.entry(a).or_insert_with(|| Vec::new()).push((c, b));
        map.entry(b).or_insert_with(|| Vec::new()).push((c, a));
    }

    let to_e = dijkstra(1, n, &map);

    let to_s = dijkstra(n, n, &map);

    let mut ans = 0;

    for v in 1..=n {
        if to_e[v] + to_s[v] == to_e[n] {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn tessoku_c14() {
        let tests = [
            TestCase(6, 7, vec![(1, 2, 15), (1, 4, 20), (2, 3, 65), (2, 5, 4), (3, 6, 50), (4, 5, 30), (5, 6, 8)], 4),
            TestCase(5, 6, vec![(1, 2, 10), (1, 3, 10), (1, 4, 10), (2, 5, 20), (3, 5, 20), (4, 5, 20)], 5),
        ];

        for TestCase(n, m, abc, expected) in tests {
            assert_eq!(run(n, m, abc), expected);
        }
    }
}
