// https://atcoder.jp/contests/abc454/tasks/abc454_c

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut map = HashMap::new();

    for (a, b) in ab {
        map.entry(a).or_insert_with(Vec::new).push(b);
    }

    let mut dequeue = VecDeque::new();
    dequeue.push_front(1);

    let mut visited = vec![false; n + 1];

    while let Some(cur) = dequeue.pop_front() {
        if visited[cur] {
            continue;
        }

        visited[cur] = true;

        let Some(next) = map.get(&cur) else {
            continue;
        };

        for n in next {
            dequeue.push_front(*n);
        }
    }

    visited
        .into_iter()
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc454_c() {
        let tests = [
            TestCase(5, 5, vec![(1, 2), (2, 3), (3, 4), (2, 4), (5, 2)], 4),
            TestCase(3, 2, vec![(2, 1), (3, 2)], 1),
            TestCase(7, 8, vec![(2, 6), (2, 5), (3, 6), (1, 6), (1, 2), (5, 6), (2, 3), (3, 7)], 6),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
