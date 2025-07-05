// https://atcoder.jp/contests/abc168/tasks/abc168_d

use std::collections::{HashMap, VecDeque};

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut hash_map = HashMap::new();

    for (a, b) in ab {
        hash_map.entry(a-1).or_insert_with(Vec::new).push(b-1);
        hash_map.entry(b-1).or_insert_with(Vec::new).push(a-1);
    }

    let mut ans = vec![0; n-1];

    let mut graph = vec![false; n];
    graph[0] = true;

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);

    while let Some(cur) = queue.pop_front() {
        let next = hash_map.get(&cur).unwrap();

        for new_i in next.iter() {
            if graph[*new_i] == true {
                continue;
            }

            graph[*new_i] = true;
            ans[*new_i-1] = cur + 1;
            queue.push_back(*new_i);
        }

    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc168_d() {
        let tests = [
            TestCase(4, 4, vec![(1, 2), (2, 3), (3, 4), (4, 2)], vec![1, 2, 2]),
            TestCase(6, 9, vec![(3, 4), (6, 1), (2, 4), (5, 3), (4, 6), (1, 5), (6, 2), (4, 5), (5, 6)], vec![6, 5, 6, 1, 1]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
