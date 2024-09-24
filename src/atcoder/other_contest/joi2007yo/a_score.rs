// https://atcoder.jp/contests/joi2007yo/tasks/joi2007yo_a

use std::cmp::max;

fn run(a: [usize; 4], b: [usize; 4]) -> usize {
    max(a.into_iter().sum(), b.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([usize; 4], [usize; 4], usize);

    #[test]
    fn test() {
        let tests = [
            TestCase([100, 80, 70, 60], [80, 70, 80, 90], 320),
            TestCase([100, 80, 70, 60], [80, 70, 60, 100], 310),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
