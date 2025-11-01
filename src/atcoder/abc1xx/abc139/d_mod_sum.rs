// https://atcoder.jp/contests/abc139/tasks/abc139_d

fn run(n: usize) -> usize {
    (1..n).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc139_d() {
        let tests = [
            TestCase(2, 1),
            TestCase(13, 78),
            TestCase(1, 0),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
