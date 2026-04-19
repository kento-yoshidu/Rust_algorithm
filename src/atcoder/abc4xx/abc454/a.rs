// https://atcoder.jp/contests/abc454/tasks/abc454_a

fn run(l: usize, r: usize) -> usize {
    r - l + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc454_a() {
        let tests = [
            TestCase(3, 5, 3),
            TestCase(1, 7, 7),
            TestCase(14, 79, 66),
        ];

        for TestCase(l, r, expected) in tests {
            assert_eq!(run(l, r), expected);
        }
    }
}
