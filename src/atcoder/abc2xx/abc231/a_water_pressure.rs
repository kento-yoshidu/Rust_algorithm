// https://atcoder.jp/contests/abc231/tasks/abc231_a

fn run(d: usize) -> f64 {
    d as f64 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64);

    #[test]
    fn abc231_a() {
        let tests = [
            TestCase(1000, 10.0),
            TestCase(50, 0.5),
            TestCase(3141, 31.41),
        ];

        for TestCase(d, expected) in tests {
            assert_eq!(run(d), expected);
        }
    }
}
