// https://atcoder.jp/contests/abc211/tasks/abc211_d

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, ab: Option<Vec<(usize, usize)>>) -> usize {
    let Some(ab) = ab else {
        return 0;
    };

    let mut vec = HashMap::new();

    let md = 1000_000_007;

    for (a, b) in ab {
        vec.entry(a).or_insert_with(Vec::new).push(b);
        vec.entry(b).or_insert_with(Vec::new).push(a);
    }

    let mut graph = vec![-1; n];
    let mut queue = VecDeque::new();
    let mut count = vec![0; n];

    graph[0] = 0;
    count[0] = 1;
    queue.push_back(1);

    while let Some(current) = queue.pop_front() {
        if let Some(next) = vec.get(&current) {
            for &next in next.iter() {
                if graph[next-1] == -1 {
                    queue.push_back(next);
                    graph[next-1] = graph[current-1] + 1;
                    count[next-1] = count[current-1];
                } else if graph[next-1] == graph[current-1] + 1 {
                    count[next-1] += count[current-1];
                    count[next-1] %= md;
                }
            }
        };
    }

    count[n-1] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Option<Vec<(usize, usize)>>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 5, Some(vec![(2, 4), (1, 2), (2, 3), (1, 3), (3, 4)]), 2),
            TestCase(4, 3, Some(vec![(1, 3), (2, 3), (2, 4)]), 1),
            TestCase(2, 0, None, 0),
            TestCase(7, 8, Some(vec![(1, 3), (1, 4), (2, 3), (2, 4), (2, 5), (2, 6), (5, 7), (6, 7)]), 4),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
