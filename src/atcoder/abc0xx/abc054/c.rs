// https://atcoder.jp/contests/abc054/tasks/abc054_c

use std::collections::HashMap;

fn dfs(n: usize, current: usize, seen: &mut Vec<bool>, graph: &HashMap<usize, Vec<usize>>) -> usize {
    seen[current] = true;

    if (1..=n).all(|i| seen[i]) {
        seen[current] = false;
        return 1;
    }

    let mut count = 0;

    for next in graph.get(&current).unwrap() {
        if seen[*next] {
            continue;
        }

        count += dfs(n, *next, seen, graph);
    }

    seen[current] = false;

    count
}

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut hash_map = HashMap::new();

    for (a, b) in ab {
        hash_map.entry(a).or_insert_with(|| Vec::new()).push(b);
        hash_map.entry(b).or_insert_with(|| Vec::new()).push(a);
    }

    dfs(n, 1, &mut vec![false; n+1], &mut hash_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc054_c() {
        let tests = [
            TestCase(3, 3, vec![(1, 2), (1, 3), (2, 3)], 2),
            TestCase(7, 7, vec![ (1, 3), (2, 7), (3, 4), (4, 5), (4, 6), (5, 6), (6, 7)], 1),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
