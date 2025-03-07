// https://atcoder.jp/contests/arc016/tasks/arc016_1

fn run(n: usize, m: usize) -> usize {
    if n == m {
        1
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 4, 1),
            TestCase(2, 1, 2)
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
