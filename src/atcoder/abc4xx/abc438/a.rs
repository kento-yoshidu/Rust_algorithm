// https://atcoder.jp/contests/abc438/tasks/abc438_a

fn run(d: usize, f: usize) -> usize {
    7 - (d - f) % 7
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc438_a() {
        let tests = [
            TestCase(365, 4, 3),
            TestCase(10, 5, 2),
        ];

        for TestCase(d, f, expected) in tests {
            assert_eq!(run(d, f), expected);
        }
    }
}
