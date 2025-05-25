// https://atcoder.jp/contests/abc112/tasks/abc112_b

fn run(_n: usize, t: usize, vec: Vec<(usize, usize)>) -> String {
    vec.iter()
        .filter(|tup| {
            tup.1 <= t
        })
        .min_by(|a, b| {
            a.0.cmp(&b.0)
        })
        .map(|tup| {
            tup.0.to_string()
        })
        .unwrap_or("TLE".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc112_b() {
        let tests = [
            TestCase(3, 70, vec![(7, 60), (1, 80), (4, 50)], "4"),
            TestCase(4, 3, vec![(1, 1000), (2, 4), (3, 1000), (4, 500)], "TLE"),
            TestCase(5, 9, vec![(25, 8), (5, 9), (4, 10), (1000, 1000), (6, 1)], "5"),
        ];

        for TestCase(n, t, vec, expected) in tests {
            assert_eq!(run(n, t, vec), expected);
        }
    }
}
