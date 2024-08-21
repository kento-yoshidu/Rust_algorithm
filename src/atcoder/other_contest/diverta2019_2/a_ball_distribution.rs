// https://atcoder.jp/contests/diverta2019-2/tasks/diverta2019_2_a

fn run(n: usize, k: usize) -> usize {
    if k == 1 {
        0
    } else {
        n - k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, 1),
            TestCase(3, 1, 0),
            TestCase(8, 5, 3),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
