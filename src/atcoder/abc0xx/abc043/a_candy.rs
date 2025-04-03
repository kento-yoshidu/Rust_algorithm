// https://atcoder.jp/contests/abc043/tasks/abc043_a

fn func(n: usize) -> usize {
    if n == 1 {
        1
    } else {
        func(n - 1) + n
    }
}

fn run(n: usize) -> usize {
    func(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 6),
            TestCase(10, 55),
            TestCase(1, 1),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
