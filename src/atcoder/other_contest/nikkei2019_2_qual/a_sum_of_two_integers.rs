// https://atcoder.jp/contests/nikkei2019-2-qual/tasks/nikkei2019_2_qual_a

fn run(n: usize) -> usize {
    (n - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 1),
            TestCase(999999, 499999),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
