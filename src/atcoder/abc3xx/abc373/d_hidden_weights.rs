// https://atcoder.jp/contests/abc373/tasks/abc373_d

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, uvw: Vec<(usize, usize, isize)>) -> Vec<isize> {
    let mut hash_map = HashMap::new();

    for (u, v, w) in uvw {
        hash_map.entry(u).or_insert_with(|| Vec::new()).push((v, w));
        hash_map.entry(v).or_insert_with(|| Vec::new()).push((u, -w));
    }

    let mut visited = vec![false; n+1];
    let mut graph = vec![0; n+1];

    for i in 1..=n {
        if visited[i] {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(i);

        while let Some(cur) = queue.pop_front() {
            if visited[cur] {
                continue;
            }

            visited[cur] = true;

            let Some(next) = hash_map.get(&cur) else {
                continue;
            };

            for &(next_v, w) in next {
                if visited[next_v] {
                    continue;
                }

                graph[next_v] = graph[cur] + w;
                queue.push_back(next_v);
            }
        }
    }

    graph.into_iter().skip(1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, isize)>, Vec<isize>);

    #[test]
    fn abc373_d() {
        let tests = [
            TestCase(3, 3, vec![(1, 2, 2), (3, 2, 3), (1, 3, -1)], vec![0, 2, -1]),
            TestCase(4, 2, vec![(2, 1, 5), (3, 4, -3)], vec![0, -5, 0, -3]),
            TestCase(5, 7, vec![(2, 1, 18169343), (3, 1, 307110901), (4, 1, 130955934), (2, 3, -288941558), (2, 5, 96267410), (5, 3, -385208968), (4, 3, -176154967)], vec![0, -18169343, -307110901, -130955934, 78098067]),
        ];

        for TestCase(n, m, uvw, expected) in tests {
            assert_eq!(run(n, m, uvw), expected);
        }
    }
}
