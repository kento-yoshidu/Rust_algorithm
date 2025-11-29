// https://atcoder.jp/contests/abc230/tasks/abc230_a

fn run(n: usize) -> String {
    if n < 42 {
        format!("AGC{:03}", n)
    } else {
        format!("AGC{:03}", n+1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc230_a() {
        let tests = [
            TestCase(42, "AGC043"),
            TestCase(19, "AGC019"),
            TestCase(1, "AGC001"),
            TestCase(50, "AGC051"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
