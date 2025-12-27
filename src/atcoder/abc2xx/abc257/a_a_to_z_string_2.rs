// https://atcoder.jp/contests/abc257/tasks/abc257_a

fn run(n: usize, x: usize) -> char {
    ('A'..).nth((x - 1) / n).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, char);

    #[test]
    fn abc257_a() {
        let tests = [
            TestCase(1, 3, 'C'),
            TestCase(2, 12, 'F'),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
