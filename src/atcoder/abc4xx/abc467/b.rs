// https://atcoder.jp/contests/abc467/tasks/abc467_b

fn run(_n: usize, abs: Vec<(usize, usize, &str)>) -> usize {
    abs.into_iter()
        .filter_map(|(a, b, s)| {
            (s == "keep").then_some(b - a)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize, &'static str)>, usize);

    #[test]
    fn abc467_b() {
        let tests = [
            TestCase(3, vec![(1, 2, "keep"), (3, 6, "take"), (5, 9, "keep")], 5),
            TestCase(8, vec![(36, 49, "take"), (38, 73, "keep"), (27, 85, "take"), (65, 71, "take"), (52, 86, "keep"), (48, 60, "keep"), (37, 98, "keep"), (5, 38, "keep")], 175),
        ];

        for TestCase(n, abs, expected) in tests {
            assert_eq!(run(n, abs), expected);
        }
    }
}
