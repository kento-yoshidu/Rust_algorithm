// https://atcoder.jp/contests/abc296/tasks/abc296_c

use std::collections::HashSet;

fn run(_n: usize, x: isize, a: Vec<isize>) -> &'static str {
    let hash_set: HashSet<isize> = a.iter().cloned().collect();

    if a.into_iter()
        .any(|num| {
            hash_set.contains(&(num - x))
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 5, vec![3, 1, 4, 1, 5, 9], "Yes"),
            TestCase(6, -4, vec![-2, -7, -1, -8, -2, -8], "No"),
            TestCase(2, 0, vec![141421356, 17320508], "Yes"),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, a), expected);
        }
    }
}
