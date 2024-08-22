// https://atcoder.jp/contests/agc020/tasks/agc020_a

fn run(_n: usize, a: usize, b: usize) -> &'static str {
    if (b - a) % 2 == 0 {
        "Alice"
    } else {
        "Borys"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, 4, "Alice"),
            TestCase(2, 1, 2, "Borys"),
            TestCase(58, 23, 42, "Borys"),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
