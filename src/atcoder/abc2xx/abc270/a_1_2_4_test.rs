// https://atcoder.jp/contests/abc270/tasks/abc270_a

fn run(a: usize, b: usize) -> usize {
    a | b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc270_a() {
        let tests = [
            TestCase(1, 2, 3),
            TestCase(5, 3, 7),
            TestCase(0, 0, 0),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
