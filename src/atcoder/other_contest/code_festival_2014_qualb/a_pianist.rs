// https://atcoder.jp/contests/code-festival-2014-qualb/tasks/code_festival_qualB_a

use std::cmp::max;

fn run(a: usize, b: usize) -> usize {
    max(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 10, 10),
            TestCase(600, 600, 600),
            TestCase(250, 200, 250),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
