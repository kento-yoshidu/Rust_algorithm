// https://atcoder.jp/contests/abc100/tasks/abc100_a

fn run(a: usize, b: usize) -> &'static str {
    if a.max(b) <= 8 {
        "Yay!"
    } else {
        ":("
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc100_a() {
        let tests = [
            TestCase(5, 4, "Yay!"),
            TestCase(8, 8, "Yay!"),
            TestCase(11, 4, ":("),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
