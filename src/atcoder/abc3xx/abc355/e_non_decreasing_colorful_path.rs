// https://atcoder.jp/contests/abc335/tasks/abc335_e

use std::collections::{BinaryHeap, HashMap};

fn dijkstra(n: usize, hash_map: &HashMap<usize, Vec<usize>>, a: &Vec<usize>) -> usize {
    let mut dist = vec![0; n+1];
    dist[1] = a[0];
    let mut count = vec![0; n+1];
    count[1] = 1;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((dist[1], 1));

    while let Some((total_point, cur_i)) = priority_queue.pop() {
        let Some(next) = hash_map.get(&cur_i) else {
            continue;
        };

        for next_i in next {
            if a[cur_i-1] >= a[*next_i-1] {
                continue;
            }

            let new_point = total_point + a[*next_i-1];

            if new_point == dist[*next_i] {
                continue;
            }

            if new_point > dist[*next_i] {
                dist[*next_i] = new_point;
                count[*next_i] = count[cur_i] + 1;
                priority_queue.push((new_point, *next_i));
            }
        }
    }

    count[n]
}

pub fn run(n: usize, _m: usize, a: Vec<usize>, uv: Vec<(usize, usize)>) -> usize {
    let mut hash_map = HashMap::new();

    for (u, v) in uv {
        hash_map.entry(u).or_insert_with(|| Vec::new()).push(v);
        hash_map.entry(v).or_insert_with(|| Vec::new()).push(u);
    }

    dijkstra(n, &hash_map, &a)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 6, vec![10, 20, 30, 40, 50], vec![(1, 2),(1, 3),(2, 5),(3, 4),(3, 5),(4, 5)], 4),
            TestCase(4, 5, vec![1, 10, 11, 4], vec![(1, 2), (1, 3), (2, 3), (2, 4), (3, 4)], 0),
            TestCase(10, 12, vec![1, 2, 3, 3, 4, 4, 4, 6, 5, 7], vec![(1, 3), (2, 9), (3, 4), (5, 6), (1, 2), (8, 9), (4, 5), (8, 10), (7, 10), (4, 6), (2, 8), (6, 7)], 5),
        ];

        for TestCase(n, m, a, uv, expected) in tests {
            assert_eq!(run(n, m, a, uv), expected);
        }
    }
}
