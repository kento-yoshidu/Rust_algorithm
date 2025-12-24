// https://atcoder.jp/contests/abc254/tasks/abc254_e

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>, _q: usize, xk: Vec<(usize, usize)>) -> Vec<usize> {
    let mut hash_map = HashMap::new();

    for (a, b) in ab {
        hash_map.entry(a).or_insert_with(Vec::new).push(b);
        hash_map.entry(b).or_insert_with(Vec::new).push(a);
    }

    let mut ans = Vec::new();

    for (x, k) in xk {
        if let None =  hash_map.get(&x) {
            ans.push(x);
            continue;
        }

        let mut graph = vec![false; n];
        let mut queue = VecDeque::new();

        queue.push_back((x, k));

        // 辿ったノードの合計
        let mut sum = x;

        while let Some((x, k)) = queue.pop_front() {
            if k == 0 {
                continue;
            }

            graph[x-1] = true;

            let next = hash_map.get(&x).unwrap();

            for n in next {
                if !graph[n-1] {
                    graph[n-1] = true;
                    queue.push_back((*n, k-1));
                    sum += *n;
                }
            }
        }

        ans.push(sum);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc254_e() {
        let tests = [
            TestCase(6, 5, vec![(2, 3), (3, 4), (3, 5), (5, 6), (2, 6)], 7, vec![(1, 1), (2, 2), (2, 0), (2, 3), (4, 1), (6, 0), (4, 3)], vec![1, 20, 2, 20, 7, 6, 20]),
        ];

        for TestCase(n, m, ab, q, xk, expected) in tests {
            assert_eq!(run(n, m, ab, q, xk), expected);
        }
    }
}
