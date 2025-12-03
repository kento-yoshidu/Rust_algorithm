// https://atcoder.jp/contests/abc179/tasks/abc179_b

fn run(_n: usize, vec: Vec<(usize, usize)>) -> &'static str {
    if vec.windows(3)
        .any(|d| {
            d.iter()
                .all(|(l, r)| {
                    l == r
                })
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc179_b() {
        let tests = [
            TestCase(5, vec![(1, 2), (6, 6), (4, 4), (3, 3), (3, 2)], "Yes"),
            TestCase(5, vec![(1, 1), (2, 2), (3, 4), (5, 5), (6, 6)], "No"),
            TestCase(6, vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)], "Yes"),
        ];

        for TestCase(n, vec, expected) in tests {
            assert_eq!(run(n, vec), expected);
        }
    }
}
