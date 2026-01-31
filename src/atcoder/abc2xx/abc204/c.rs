// https://atcoder.jp/contests/abc204/tasks/abc204_c

use std::collections::VecDeque;

fn bfs(n: usize, start: usize, map: &Vec<Vec<usize>>) -> usize {
    let mut count = 1;

    let mut deque = VecDeque::new();
    deque.push_front(start);

    let mut visited = vec![false; n+1];
    visited[start] = true;

    while let Some(cur) =  deque.pop_front() {
        for i in map[cur].iter() {
            if visited[*i] {
                continue;
            }

            visited[*i] = true;
            count += 1;

            deque.push_back(*i);
        }
    }

    count
}

fn run(n: usize, _m: usize, ab: Option<Vec<(usize, usize)>>) -> usize {
    if ab.is_none() {
        return n;
    }

    let mut map = vec![Vec::new(); n+1];

    for (a, b) in ab.unwrap() {
        map[a].push(b);
    }

    (1..=n).into_iter()
        .map(|i| bfs(n, i, &map))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<(usize, usize)>>, usize);

    #[test]
    fn abc204_c() {
        let tests = [
            TestCase(3, 3, Some(vec![(1, 2), (2, 3), (3, 2)]), 7),
            TestCase(3, 0, None, 3),
            TestCase(4, 4, Some(vec![(1, 2), (2, 3), (3, 4), (4, 1)]), 16),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
