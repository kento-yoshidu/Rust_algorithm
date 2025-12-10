// https://atcoder.jp/contests/abc190/tasks/abc190_b

fn run(_n: usize, s: usize, d: usize, v: Vec<(usize, usize)>) -> &'static str {
    if v.into_iter()
        .any(|(x, y)| {
            x < s && y > d
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc190_b() {
        let tests = [
            TestCase(4, 9, 9, vec![(5, 5), (15, 5), (5, 15), (15, 15)], "Yes"),
            TestCase(3, 691, 273, vec![(691, 997), (593, 273), (691, 273)], "No"),
            TestCase(7, 100, 100, vec![(10, 11), (12, 67), (192, 79), (154, 197), (142, 158), (20, 25), (17, 108)], "Yes"),
        ];

        for TestCase(n, s, d, v, expected) in tests {
            assert_eq!(run(n, s, d, v), expected);
        }
    }
}
