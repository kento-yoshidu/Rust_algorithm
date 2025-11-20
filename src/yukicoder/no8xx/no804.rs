// https://yukicoder.me/problems/no/804

use std::cmp::min;

fn run(a: usize, b: usize, c: usize, d: usize) -> usize {
    min(a, min(b / c, d / (c + 1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn yuki_804() {
        let tests = [
            TestCase(3, 8, 4, 12, 2),
            TestCase(3, 8, 100, 12, 0),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
