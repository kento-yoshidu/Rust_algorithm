// https://atcoder.jp/contests/abc270/tasks/abc270_c

use std::collections::HashMap;

fn dfs(
    map: &HashMap<&usize, Vec<&usize>>,
    ans: &mut Vec<usize>,
    seen: &mut Vec<bool>,
    current: usize,
    y: usize,
) -> bool {
    if current == y {
        return true;
    }

    for &next in map.get(&current).unwrap_or(&vec![]) {
        if seen[*next] {
            continue;
        }

        seen[*next] = true;
        ans.push(*next);

        if dfs(map, ans, seen, *next, y) {
            return true;
        }

        ans.pop();
    }

    false
}

fn run(n: usize, x: usize, y: usize, uv: Vec<(usize, usize)>) -> Vec<usize> {
    let mut hash_map = HashMap::new();

    for (u, v) in uv.iter() {
        hash_map.entry(u).or_insert_with(|| Vec::new()).push(v);
        hash_map.entry(v).or_insert_with(|| Vec::new()).push(u);
    }

    let mut seen = vec![false; n+1];
    seen[x] = true;

    let mut ans = vec![x];

    dfs(&hash_map, &mut ans, &mut seen, x, y);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc270_c() {
        let tests = [
            TestCase(5, 2, 5, vec![(1, 2), (1, 3), (3, 4), (3, 5)], vec![2, 1, 3, 5]),
            TestCase(6, 1, 2, vec![(3, 1), (2, 5), (1, 2), (4, 1), (2, 6)], vec![1, 2]),
        ];

        for TestCase(n, x, y, uv, expected) in tests {
            assert_eq!(run(n, x, y, uv), expected);
        }
    }
}
