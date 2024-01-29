// https://atcoder.jp/contests/abc030/tasks/abc030_a

use std::cmp::Ordering::*;

pub fn run(a: f64, b: f64, c: f64, d: f64) -> &'static str {
    match (b / a).partial_cmp(&(d / c)) {
        Some(Greater) => "TAKAHASHI",
        Some(Less) => "AOKI",
        Some(Equal) => "DRAW",
        None => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, f64, f64, f64, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5.0, 2.0, 6.0, 3.0, "AOKI"),
            TestCase(100.0, 80.0, 100.0, 73.0, "TAKAHASHI"),
            TestCase(66.0, 30.0, 55.0, 25.0, "DRAW"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
