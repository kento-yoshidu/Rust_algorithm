// https://atcoder.jp/contests/diverta2019/tasks/diverta2019_a

fn run(n: usize, k: usize) -> usize {
    n - k + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, 2),
            TestCase(13, 3, 11),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
