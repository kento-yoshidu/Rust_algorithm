// https://atcoder.jp/contests/abc128/tasks/abc128_a

fn run(a: usize, p: usize) -> usize {
    let piece = p + a * 3;

    piece / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc128_a() {
        let tests = [
            TestCase(1, 3, 3),
            TestCase(0, 1, 0),
            TestCase(32, 21, 58),
        ];

        for TestCase(a, p, expected) in tests {
            assert_eq!(run(a, p), expected);
        }
    }
}
