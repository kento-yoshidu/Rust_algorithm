// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_an

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<isize> {
    let mut hash_map = HashMap::new();

    for (a, b) in ab {
        hash_map.entry(a-1).or_insert_with(Vec::new).push(b-1);
        hash_map.entry(b-1).or_insert_with(Vec::new).push(a-1);
    }

    let mut graph = vec![-1; n];
    graph[0] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(0);

    while let Some(cur) = queue.pop_front() {
        let Some(next) = hash_map.get(&cur) else {
            continue;
        };

        for next in next.iter() {
            if graph[*next] != -1 {
                continue;
            }

            graph[*next] = graph[cur] + 1;
            queue.push_back(*next);
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, vec![(1, 3), (2, 3)], vec![0, 2, 1]),
            TestCase(6 , 6, vec![(1, 4), (2, 3), (3, 4), (5, 6), (1, 2), (2, 4)], vec![0, 1, 2, 1, -1, -1]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
