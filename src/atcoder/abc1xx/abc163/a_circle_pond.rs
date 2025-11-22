// https://atcoder.jp/contests/abc163/tasks/abc163_a

fn run(r: usize) -> f64 {
    r as f64 * 2.0 * std::f64::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64);

    #[test]
    fn abc163_a() {
        let tests = [
            TestCase(1, 6.28318530717958623200),
            TestCase(73, 458.67252742410977361942),
        ];

        for TestCase(r, expected) in tests {
            assert_eq!(run(r), expected);
        }
    }
}
