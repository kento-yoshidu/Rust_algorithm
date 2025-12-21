// https://atcoder.jp/contests/abc211/tasks/abc211_a

fn run(a: usize, b: usize) -> f64 {
    ((a - b) as f64 / 3.0) + b as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, f64);

    #[test]
    fn abc211_a() {
        let tests = [
            TestCase(130, 100, 110.0),
            TestCase(300, 50, 133.33333333333331),
            TestCase(123, 123, 123.0),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
