// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_am

use std::collections::HashMap;

fn dfs(seen: &mut Vec<bool>, map: &HashMap<usize, Vec<usize>>, pos: usize) {
    seen[pos] = true;

    if let Some(vec) = map.get(&pos) {
        for v in vec.iter() {
            if !seen[*v] {
                dfs(seen, map, *v)
            }
        }
    }
}

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> &'static str {
    let mut hash_map = HashMap::new();

    for (a, b) in ab.into_iter() {
        hash_map.entry(a).or_insert_with(|| Vec::new()).push(b);
        hash_map.entry(b).or_insert_with(|| Vec::new()).push(a);
    }

    let mut seen = vec![false; n+1];

    dfs(&mut seen, &hash_map, 1);

    if seen.into_iter()
        .skip(1)
        .any(|b| !b) {
            "The graph is not connected."
        } else {
            "The graph is connected."
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn tessoku_a62() {
        let tests = [
            TestCase(3, 2, vec![(1, 3), (2, 3)], "The graph is connected."),
            TestCase(6, 6, vec![(1, 4), (2, 3), (3, 4), (5, 6), (1, 2), (2, 4)], "The graph is not connected."),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
