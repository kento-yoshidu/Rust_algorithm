// https://atcoder.jp/contests/abc054/tasks/abc054_a

fn run(a: usize, b: usize) -> &'static str {
    if a == b {
        return "Draw";
    }

    if a == 1 {
        return "Alice";
    }

    if b == 1 {
        return "Bob";
    }

    if a > b {
        "Alice"
    } else {
        "Bob"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, 6, "Alice"),
            TestCase(1, 1, "Draw"),
            TestCase(13, 1, "Bob"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
