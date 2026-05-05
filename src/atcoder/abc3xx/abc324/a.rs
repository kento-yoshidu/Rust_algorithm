// https://atcoder.jp/contests/abc324/tasks/abc324_a

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>) -> &'static str {
    if a.into_iter().all_equal() {
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
    fn abc324_a() {
        let tests = [
            TestCase(3, vec![3, 2, 4], "No"),
            TestCase(4, vec![3, 3, 3, 3], "Yes"),
            TestCase(4, vec![73, 8, 55, 26, 97, 48, 37, 47, 35, 55], "No"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
