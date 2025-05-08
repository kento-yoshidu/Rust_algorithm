// https://atcoder.jp/contests/abc067/tasks/abc067_a

fn run(a: usize, b: usize) -> &'static str {
    if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
        "Possible"
    } else {
        "Impossible"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 5, "Possible"),
            TestCase(1, 1, "Impossible"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
