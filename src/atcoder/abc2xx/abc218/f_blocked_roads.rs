// https://atcoder.jp/contests/abc218/tasks/abc218_f

use std::collections::{HashMap, VecDeque};

fn bfs(n: usize, start: usize, hash_map: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut dist = vec![std::usize::MAX; n+1];
    dist[start] = 0;

    let mut queue = VecDeque::new();
    queue.push_front(start);

    while let Some(cur) = queue.pop_front() {
        let Some(next) = hash_map.get(&cur) else {
            continue;
        };

        for next in next {
            if dist[*next] != std::usize::MAX {
                continue;
            }

            dist[*next] = dist[cur] + 1;

            queue.push_back(*next);
        }
    }

    dist
}

fn run(n: usize, _m: usize, st: Vec<(usize, usize)>) -> Vec<isize> {
    let mut graph = HashMap::new();

    for &(s, t) in &st {
        graph.entry(s).or_insert_with(Vec::new).push(t);
    }

    let dist = bfs(n, 1, &graph);

    let mut ans = Vec::new();

    for &(s, t) in &st {
        if dist[s] + 1 != dist[t] {
            ans.push(dist[n] as isize);
            continue;
        }

        let mut graph_clone = graph.clone();
        graph_clone.get_mut(&s).map(|v| v.retain(|&x| x != t));

        let dist_after_removal = bfs(n, 1, &graph_clone);

        let dist: isize = if dist_after_removal[n] == std::usize::MAX {
            -1
        } else {
            dist_after_removal[n] as isize
        };

        ans.push(dist);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<isize>);

    #[test]
    fn abc218_f() {
        let tests = [
            TestCase(3, 3, vec![(1, 2), (1, 3), (2, 3)], vec![1, 2, 1]),
            TestCase(4, 4, vec![(1, 2), (2, 3), (2, 4), (3, 4)], vec![-1, 2, 3, 2]),
            TestCase(5, 10, vec![(1, 2), (1, 4), (1, 5), (2, 1), (2, 3), (3, 1), (3, 2), (3, 5), (4, 2), (4, 3)], vec![ 1, 1, 3, 1, 1, 1, 1, 1, 1, 1]),
            TestCase(4, 1, vec![(1, 2)], vec![-1]),
        ];

        for TestCase(n, m, st, expected) in tests {
            assert_eq!(run(n, m, st), expected);
        }
    }
}
