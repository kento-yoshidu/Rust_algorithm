// https://atcoder.jp/contests/abc145/tasks/abc145_a

fn run(r: usize) -> usize {
    r * r
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc145_a() {
        let tests = [
            TestCase(2, 4),
            TestCase(100, 10000),
        ];

        for TestCase(r, expected) in tests {
            assert_eq!(run(r), expected);
        }
    }
}
