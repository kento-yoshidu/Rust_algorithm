// https://atcoder.jp/contests/abc413/tasks/abc413_a

fn run(_n: usize, m: usize, a: Vec<usize>) -> &'static str {
    if m >= a.into_iter().sum() {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn abc413_a() {
        let tests = [
            TestCase(5, 15, vec![3, 1, 4, 1, 5], "Yes"),
            TestCase(5, 5, vec![3, 1, 4, 1, 5], "No"),
            TestCase(1, 10000, vec![100], "Yes"),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
