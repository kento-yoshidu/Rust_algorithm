// https://atcoder.jp/contests/abc100/tasks/abc100_b

fn run(d: usize, n: usize) -> usize {
    if n == 100 {
        (100_u32.pow(d as u32)) as usize * 101
    } else {
        (100_u32.pow(d as u32)) as usize * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(0, 5, 5),
            TestCase(1, 11, 1100),
            TestCase(2, 85, 850000),
            TestCase(0, 100, 101),
        ];

        for TestCase(d, n, expected) in tests {
            assert_eq!(run(d, n), expected);
        }
    }
}
