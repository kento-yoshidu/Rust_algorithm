// https://atcoder.jp/contests/abc407/tasks/abc407_b

use std::cmp::{min, max};

fn run(x: usize, y: usize) -> f64 {
    let mut count = 0;

    for i in 1..=6 {
        for j in 1..=6 {
            let l = max(i, j);
            let s = min(i, j);

            if i + j >= x || l - s >= y {
                count += 1;
            }
        }
    }

    count as f64 / 36.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, f64);

    #[test]
    fn abc407_b() {
        let tests = [
            TestCase(9, 3, 0.5555555555555556),
            TestCase(13, 6, 0.0),
            TestCase(10, 3, 0.5),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
