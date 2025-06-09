// https://atcoder.jp/contests/abc376/tasks/abc376_d

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> isize {
    let mut hash_map = HashMap::new();

    for (a, b) in ab {
        hash_map.entry(a).or_insert_with(|| Vec::new()).push(b);
    }

    let mut graph = vec![false; n+1];

    let mut queue = VecDeque::new();
    queue.push_back((1, 0));

    while let Some((cur, count)) = queue.pop_front() {
        let Some(next) = hash_map.get(&cur) else {
            continue;
        };

        for next in next.iter() {
            if *next == 1 {
                return count + 1;
            }

            if graph[*next] {
                continue;
            }

            graph[*next] = true;
            queue.push_back((*next, count+1));
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 3, vec![(1, 2), (2, 3), (3, 1)], 3),
            TestCase(3, 2, vec![(1, 2), (2, 3)], -1),
            TestCase(6, 9, vec![(6, 1), (1, 5), (2, 6), (2, 1), (3, 6), (4, 2), (6, 4), (3, 5), (5, 4)], 4),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
