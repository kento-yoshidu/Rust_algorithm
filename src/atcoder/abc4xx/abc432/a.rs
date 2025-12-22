// https://atcoder.jp/contests/abc432/tasks/abc432_a

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize) -> usize {
    let sorted: Vec<usize> = [a, b, c].into_iter().sorted().collect();

    sorted[2] * 100 + sorted[1] * 10 + sorted[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc432_a() {
        let tests = [
            TestCase(3, 2, 4, 432),
            TestCase(7, 7, 7, 777),
            TestCase(9, 1, 9, 991),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected)
        }
    }
}
