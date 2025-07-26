// https://atcoder.jp/contests/abc016/tasks/abc016_3

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut hash_map = HashMap::new();

    for (a, b) in ab {
        hash_map.entry(a).or_insert_with(Vec::new).push(b);
        hash_map.entry(b).or_insert_with(Vec::new).push(a);
    }

    let mut ans = Vec::new();

    for i in 1..=n {
        let mut graph = vec![false; n+1];
        let mut queue = VecDeque::new();

        // 友達の友達
        let mut count = 0;

        graph[i] = true;
        queue.push_back((i, 2));

        while let Some((x, k)) = queue.pop_front() {
            if k == 0 {
                continue;
            }

            let Some(next) = hash_map.get(&x) else {
                continue;
            };

            for &n in next {
                if !graph[n] {
                    graph[n] = true;
                    if k == 1 {
                        count += 1;
                    }
                    queue.push_back((n, k-1));
                }
            }
        }

        ans.push(count);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc016_c() {
        let tests = [
            TestCase(3, 2, vec![(1, 2), (2, 3)], vec![1, 0, 1]),
            TestCase(3, 3, vec![(1, 2), (1, 3), (2, 3)], vec![0, 0, 0]),
            TestCase(8, 12, vec![(1, 6),(1, 7),(1, 8),(2, 5),(2, 6),(3, 5),(3, 6),(4, 5),(4, 8),(5, 6),(5, 7),(7, 8)], vec![4, 4, 4, 5, 2, 3, 4, 2]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
