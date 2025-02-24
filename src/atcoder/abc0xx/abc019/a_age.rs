// https://atcoder.jp/contests/abc019/tasks/abc019_1

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize) -> usize {
    vec![a, b, c]
        .into_iter()
        .sorted()
        .skip(1)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 3, 4, 3),
            TestCase(5, 100, 5, 5),
            TestCase(3, 3, 3, 3),
            TestCase(3, 3, 4, 3),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
