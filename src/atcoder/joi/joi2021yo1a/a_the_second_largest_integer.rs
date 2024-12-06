// https://atcoder.jp/contests/joi2021yo1a/tasks/joi2021_yo1a_a

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize) -> usize {
    [a, b, c]
        .into_iter()
        .sorted()
        .collect::<Vec<usize>>()
        [1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 5, 3, 5),
            TestCase(1, 3, 3, 3),
            TestCase(100, 100, 100, 100),
            TestCase(29, 83, 1, 29),
        ];

        for TestCase(a, b , c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
