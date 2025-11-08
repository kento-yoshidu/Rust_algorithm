// https://atcoder.jp/contests/abc200/tasks/abc200_a

fn run(n: usize) -> usize {
    (n as f64 / 100.0).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc200_a() {
        let tests = [
            TestCase(2021, 21),
            TestCase(200, 2),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
