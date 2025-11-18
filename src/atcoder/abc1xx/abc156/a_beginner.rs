// https://atcoder.jp/contests/abc156/tasks/abc156_a

fn run(n: usize, r: usize) -> usize {
    if n >= 10 {
        r
    } else {
        (10 - n) * 100 + r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc156_a() {
        let tests = [
            TestCase(2, 2919, 3719),
            TestCase(22, 3051, 3051),
        ];

        for TestCase(n, r, expected) in tests {
            assert_eq!(run(n, r), expected);
        }
    }
}
