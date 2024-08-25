// https://atcoder.jp/contests/abc185/tasks/abc185_c

fn run(l: usize) -> usize {
    let mut ans = 1;

    for i in 1..12 {
        ans *= l-i;
        ans /= i;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(12, 1),
            TestCase(13, 12),
            TestCase(17, 4368),
        ];

        for TestCase(l, expected) in tests {
            assert_eq!(run(l), expected);
        }
    }
}
