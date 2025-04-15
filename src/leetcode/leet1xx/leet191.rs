// https://leetcode.com/problems/number-of-1-bits/description/

fn rec(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        (n & 1) + rec(n >> 1)
    }
}

fn run(n: usize) -> usize {
    rec(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(11, 3),
            TestCase(128, 1),
            TestCase(2147483645, 30),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
