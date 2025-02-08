// https://atcoder.jp/contests/abc086/tasks/abc086_a

fn run(a: usize, b: usize) -> &'static str {
    if (a * b) % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, "Even"),
            TestCase(3, 3, "Odd"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
