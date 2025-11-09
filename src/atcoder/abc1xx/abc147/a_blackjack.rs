// https://atcoder.jp/contests/abc147/tasks/abc147_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a + b + c >= 22 {
        "bust"
    } else {
        "win"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn abc147_a() {
        let tests = [
            TestCase(5, 7, 9, "win"),
            TestCase(13, 7, 2, "bust"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
