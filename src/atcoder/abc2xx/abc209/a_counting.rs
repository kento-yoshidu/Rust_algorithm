//https://atcoder.jp/contests/abc209/tasks/abc209_a

fn run(a: usize, b: usize) -> usize {
    (a..=b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc209_a() {
        let tests = [
            TestCase(2, 4, 3),
            TestCase(10, 100, 91),
            TestCase(3, 2, 0),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
