// https://atcoder.jp/contests/abc017/tasks/abc017_1

fn run(se: Vec<(usize, usize)>) -> usize {
    se.into_iter()
        .map(|t| {
            t.0 * t.1
        })
        .sum::<usize>() / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(usize, usize)>, usize);

    #[test]
    fn abc017_a() {
        let tests = [
            TestCase(vec![(50, 7), (40, 8), (30, 9)], 94),
            TestCase(vec![(990, 10), (990, 10), (990, 10)], 2970),
        ];

        for TestCase(se, expected) in tests {
            assert_eq!(run(se), expected);
        }
    }
}
