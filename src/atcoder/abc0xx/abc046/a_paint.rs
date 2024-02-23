// https://atcoder.jp/contests/abc046/tasks/abc046_a

use std::collections::HashSet;

pub fn run(a: usize, b: usize, c: usize) -> usize {
    let vec = vec![a, b, c];

    let u: HashSet<usize> = vec.into_iter().collect();

    u.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 1, 2, 3),
            TestCase(3, 3, 33, 2),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
