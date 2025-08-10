// https://atcoder.jp/contests/abc125/tasks/abc125_a

fn run(a: usize, b: usize, t: usize) -> usize {
    (t / a) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc125_a() {
        let tests = [
            TestCase(3, 5, 7, 10),
            TestCase(3, 2, 9, 6),
            TestCase(20, 20, 19, 0),
        ];

        for TestCase(a, b, t, expected) in tests {
            assert_eq!(run(a, b, t), expected);
        }
    }
}
