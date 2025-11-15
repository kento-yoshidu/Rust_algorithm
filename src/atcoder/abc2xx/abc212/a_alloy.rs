// https://atcoder.jp/contests/abc212/tasks/abc212_a

fn run(a: usize, b: usize) -> &'static str {
    if a > 0 && b > 0 {
        "Alloy"
    } else if b == 0 {
        "Gold"
    } else {
        "Silver"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc212_a() {
        let tests = [
            TestCase(50, 50, "Alloy"),
            TestCase(100, 0, "Gold"),
            TestCase(0, 100, "Silver"),
            TestCase(100, 2, "Alloy"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
