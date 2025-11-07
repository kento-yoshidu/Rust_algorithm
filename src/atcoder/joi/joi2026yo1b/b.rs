// https://atcoder.jp/contests/joi2026yo1b/tasks/joi2026_yo1b_b

use std::cmp::min;

fn run(x: usize, y: usize, z: usize) -> usize {
    if min(x, y) <= z {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn joi2026yo1b_b() {
        let tests = [
            TestCase(7, 2, 3, 1),
        ];

        for TestCase(x, y, z, expected) in tests {
            assert_eq!(run(x, y, z), expected);
        }
    }
}
