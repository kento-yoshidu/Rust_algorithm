// https://atcoder.jp/contests/abc277/tasks/abc277_c

use std::collections::{HashMap, HashSet};

fn dfs(current: usize, seen: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>) -> usize {
    let mut ans = current;
    seen.insert(ans);

    for tmp in graph.get(&current).unwrap() {
        if seen.contains(tmp) {
            continue;
        }

        ans = ans.max(dfs(*tmp, seen, graph));
    }

    ans
}

pub fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut hash_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for &(a, b) in ab.iter() {
        hash_map.entry(a).or_insert_with(|| Vec::new()).push(b);
        hash_map.entry(b).or_insert_with(|| Vec::new()).push(a);
    }

    let Some(_) = hash_map.get(&1) else {
        return 1;
    };

    dfs(1, &mut HashSet::new(), &hash_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![(1, 4), (4, 3), (4, 10), (8, 3), (1, 2)], 10),
            TestCase(6, vec![(1, 3), (1, 5), (1, 12), (3, 5), (3, 12), (5, 12)], 12),
            TestCase(3, vec![(500000000, 600000000), (600000000, 700000000), (700000000, 800000000)], 1),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
