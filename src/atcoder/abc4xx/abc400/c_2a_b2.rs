// https://atcoder.jp/contests/abc400/tasks/abc400_c

use num_integer::sqrt;

fn run(n: usize) -> usize {
    (sqrt(n / 2) + sqrt(n / 4)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(20, 5),
            TestCase(400, 24),
            TestCase(1234567890, 42413),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
