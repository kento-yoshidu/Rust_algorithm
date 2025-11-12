// https://atcoder.jp/contests/abc205/tasks/abc205_b

use std::collections::HashSet;

fn run(n: usize, a: Vec<usize>) -> &'static str {
    let hash = HashSet::<usize>::from_iter(a);

    if hash.len() == n {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn abc205_b() {
        let tests = [
            TestCase(5, vec![3, 1, 2, 4, 5], "Yes"),
            TestCase(6, vec![3, 1, 4, 1, 5, 2], "No"),
            TestCase(3, vec![1, 2, 3], "Yes"),
            TestCase(1, vec![1], "Yes"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
