// https://atcoder.jp/contests/abc262/tasks/abc262_a

fn run(y: usize) -> usize {
    (y..)
        .find(|y| y % 4 == 2)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc262_a() {
        let tests = [
            TestCase(2022, 2022),
            TestCase(2023, 2026),
            TestCase(3000, 3002),
        ];

        for TestCase(y, expected) in tests {
            assert_eq!(run(y), expected);
        }
    }
}
