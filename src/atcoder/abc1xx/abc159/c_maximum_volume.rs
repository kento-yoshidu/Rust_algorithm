// https://atcoder.jp/contests/abc159/tasks/abc159_c

fn run(l: usize) -> f64 {
    ((l as f64 / 3.0) as f64).powf(3.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64);

    #[test]
    fn abc159_c() {
        let tests = [
            TestCase(3, 1.0),
            TestCase(999, 36926037.0),
        ];

        for TestCase(l, expected) in tests {
            assert_eq!(run(l), expected);
        }
    }
}
