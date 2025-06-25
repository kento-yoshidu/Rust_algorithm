// https://atcoder.jp/contests/abc117/tasks/abc117_a

fn run(t: i32, x: i32) -> f64 {
    t as f64 / x as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, f64);

    #[test]
    fn abc117_a() {
        let tests = [
            TestCase(8, 3, 2.6666666666666665),
            TestCase(99, 1, 99.0000000000),
            TestCase(1, 100, 0.01),
        ];

        for TestCase(t, x, expected) in tests {
            assert_eq!(run(t, x), expected);
        }
    }
}
