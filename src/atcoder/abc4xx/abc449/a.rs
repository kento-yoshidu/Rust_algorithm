// https://atcoder.jp/contests/abc449/tasks/abc449_a

use std::f64::consts::PI;

fn run(d: usize) -> f64 {
    (d as f64 / 2.0) * (d as f64 / 2.0) * PI
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64);

    #[test]
    fn abc449_a() {
        let tests = [
            TestCase(2, 3.141592653589793),
            TestCase(7, 38.48451000647496),
            TestCase(98, 7542.9639612690935),
        ];

        for TestCase(d, expected) in tests {
            assert_eq!(run(d), expected);
        }
    }
}
