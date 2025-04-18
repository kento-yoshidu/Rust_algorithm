// https://atcoder.jp/contests/joi2020yo1b/tasks/joi2020_yo1b_a

use std::cmp::min;

fn run(a: usize, b: usize, c: usize) -> usize {
    let min = min(a, min(b, c));

    [a, b, c]
        .into_iter()
        .sum::<usize>()
        - min
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(70, 80, 90, 170),
            TestCase(70, 100, 70, 170),
            TestCase(70, 70, 70, 140),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
