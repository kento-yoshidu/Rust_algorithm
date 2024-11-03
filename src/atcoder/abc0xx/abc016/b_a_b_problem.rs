// https://atcoder.jp/contests/abc016/tasks/abc016_2

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if a + b == c && a - b == c {
        "?"
    } else if a + b == c {
        "+"
    } else if a - b == c {
        "-"
    } else {
        "!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 0, 1, "?"),
            TestCase(1, 1, 2, "+"),
            TestCase(1, 1, 0, "-"),
            TestCase(1, 1, 1, "!"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
