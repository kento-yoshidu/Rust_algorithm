// https://atcoder.jp/contests/abc274/tasks/abc274_a

fn run(a: f64, b: f64) -> String {
    format!("{:.3}", b / a)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, f64, &'static str);

    #[test]
    fn abc274_a() {
        let tests = [
            TestCase(7.0, 4.0, "0.571"),
            TestCase(7.0, 3.0, "0.429"),
            TestCase(2.0, 1.0, "0.500"),
            TestCase(10.0, 10.0, "1.000"),
            TestCase(1.0, 0.0, "0.000"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
