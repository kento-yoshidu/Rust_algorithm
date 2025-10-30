// https://atcoder.jp/contests/abc185/tasks/abc185_c

fn run(l: usize) -> usize {
    let mut ans = 1;

    for i in 1..12 {
        ans *= l-i;
        ans /= i;
    }

    ans
}

fn run2(l: usize) -> usize {
    (1..12)
        .fold(1, |acc, i| {
            acc * (l - i) / i
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc185_c() {
        let tests = [
            TestCase(12, 1),
            TestCase(13, 12),
            TestCase(17, 4368),
        ];

        for TestCase(l, expected) in tests {
            assert_eq!(run(l), expected);
            assert_eq!(run2(l), expected);
        }
    }
}
