// https://atcoder.jp/contests/abc034/tasks/abc034_a

fn run(x: usize, y: usize) -> &'static str {
    if x < y {
        "Better"
    } else {
        "Worse"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(12, 34, "Better"),
            TestCase(98, 56, "Worse"),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
