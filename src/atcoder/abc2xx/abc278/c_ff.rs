// https://atcoder.jp/contests/abc278/tasks/abc278_c

use std::collections::HashSet;

fn run(_n: usize, _q: usize, tab: &Vec<(usize, usize, usize)>) -> Vec<&'static str> {
    let mut hash_set = HashSet::new();

    let mut ans = Vec::new();

    for (t, a, b) in tab {
        match t {
            1 => {
                hash_set.insert((a, b));
            },
            2 => {
                hash_set.remove(&(a, b));
            },
            3 => {
                if hash_set.get(&(a, b)).is_some() && hash_set.get(&(b, a)).is_some() {
                    ans.push("Yes");
                } else {
                    ans.push("No");
                }
            },
            _ => unreachable!(),
        }
    }

    ans
}

fn run2(_n: usize, _q: usize, tab: &Vec<(usize, usize, usize)>) -> Vec<&'static str> {
    let mut hash_set = HashSet::new();

    tab.into_iter()
        .filter_map(|(t, a, b)| {
            match t {
                1 => {
                    hash_set.insert((a, b));
                    None
                },
                2 => {
                    hash_set.remove(&(a, b));
                    None
                },
                3 => {
                    if hash_set.contains(&(a, b)) && hash_set.contains(&(b, a)) {
                        Some("Yes")
                    } else {
                        Some("No")
                    }
                },
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 9, vec![(1, 1, 2), (3, 1, 2), (1, 2, 1), (3, 1, 2), (1, 2, 3), (1, 3, 2), (3, 1, 3), (2, 1, 2), (3, 1, 2)], vec!["No", "Yes", "No", "No"]),
            TestCase(2, 8, vec![(1, 1, 2), (1, 2, 1), (3, 1, 2), (1, 1, 2), (1, 1, 2), (1, 1, 2), (2, 1, 2), (3, 1, 2)], vec!["Yes", "No"]),
            TestCase(10, 30, vec![(3, 1, 6), (3, 5, 4), (1, 6, 1), (3, 1, 7), (3, 8, 4), (1, 1, 6), (2, 4, 3), (1, 6, 5), (1, 5, 6), (1, 1, 8), (1, 8, 1), (2, 3, 10), (1, 7, 6), (3, 5, 6), (1, 6, 7), (3, 6, 7), (1, 9, 5), (3, 8, 6), (3, 3, 8), (2, 6, 9), (1, 7, 1), (3, 10, 8), (2, 9, 2), (1, 10, 9), (2, 6, 10), (2, 6, 8), (3, 1, 6), (3, 1, 8), (2, 8, 5), (1, 9, 10)], vec!["No", "No", "No", "No", "Yes", "Yes", "No", "No", "No", "Yes", "Yes"]),
        ];

        for TestCase(n, q, tab, expected) in tests {
            assert_eq!(run(n, q, &tab), expected);
            assert_eq!(run2(n, q, &tab), expected);
        }
    }
}
