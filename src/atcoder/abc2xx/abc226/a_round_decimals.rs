// https://atcoder.jp/contests/abc226/tasks/abc226_a

fn run(x: f64) -> usize {
    x.round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, usize);

    #[test]
    fn abc226_a() {
        let tests = [
            TestCase(3.456, 3),
            TestCase(99.500, 100),
            TestCase(0.000, 0),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
