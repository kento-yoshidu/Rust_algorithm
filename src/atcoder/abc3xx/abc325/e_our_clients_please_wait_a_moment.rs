// https://atcoder.jp/contests/abc325/tasks/abc325_e

use std::collections::BinaryHeap;
use std::cmp::Reverse;

const INF: usize = std::usize::MAX;

fn dijkstra(
    n: usize,
    start: usize,
    a: usize,
    b: usize,
    c: usize,
    d: &Vec<Vec<usize>>,
    is_forward: bool
) -> Vec<usize> {
    let mut dist = vec![INF; n+1];

    dist[start] = 0;

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, start)));

    while let Some(Reverse((cur_cost, cur_i))) = priority_queue.pop() {
        if cur_cost > dist[cur_i] {
            continue;
        }

        for next in 1..=n {
            if next == cur_i {
                continue;
            }

            let new_cost =
                if is_forward {
                    cur_cost + d[cur_i-1][next-1] * a
                } else {
                    cur_cost + (d[cur_i-1][next-1] * b) + c
                };

            if new_cost < dist[next] {
                dist[next] = new_cost;
                priority_queue.push(Reverse((new_cost, next)));
            }
        }
    }

    dist
}

fn run(n: usize, a: usize, b: usize, c: usize, d: Vec<Vec<usize>>) -> usize {
    let dijk = dijkstra(n, 1, a, b, c, &d, true);
    let dijk2 = dijkstra(n, n, a, b, c, &d, false);

    (1..=n)
        .map(|i| dijk[i] + dijk2[i])
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<Vec<usize>>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 8, 5, 13, vec![vec![0, 6, 2, 15], vec![6, 0, 3, 5], vec![2, 3, 0, 13], vec![15, 5, 13, 0]], 78),
            TestCase(3, 1, 1000000, 1000000, vec![vec![0, 10, 1], vec![10, 0, 10], vec![1, 10, 0]], 1),
            TestCase(5, 954257, 954213, 814214, vec![vec![0, 84251, 214529, 10017, 373342], vec![84251, 0, 91926, 32336, 164457], vec![214529, 91926, 0, 108914, 57762], vec![10017, 32336, 108914, 0, 234705], vec![373342, 164457, 57762, 234705, 0]], 168604826785),
        ];

        for TestCase(n, a, b, c, d, expected) in tests {
            assert_eq!(run(n, a, b, c, d), expected);
        }
    }
}
