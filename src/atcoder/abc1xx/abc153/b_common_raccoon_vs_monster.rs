// https://atcoder.jp/contests/abc153/tasks/abc153_b

fn run(h: usize, _n: usize, a: Vec<usize>) -> &'static str {
    if h <= a.into_iter().sum() {
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
    fn abc153_b() {
        let tests = [
            TestCase(10, 3, vec![4, 5, 6], "Yes"),
            TestCase(20, 3, vec![4, 5, 6], "No"),
            TestCase(210, 5, vec![31, 41, 59, 26, 53], "Yes"),
            TestCase(211, 5, vec![31, 41, 59, 26, 53], "No"),
        ];

        for TestCase(h, n, a, expected) in tests {
            assert_eq!(run(h, n, a), expected);
        }
    }
}
