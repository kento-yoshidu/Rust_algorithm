// https://atcoder.jp/contests/abc035/tasks/abc035_a

fn run(h: usize, w: usize) -> &'static str {
    if h / 4 == w / 3 {
        "4:3"
    } else {
        "16:9"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, "4:3"),
            TestCase(16, 9, "16:9"),
            TestCase(28, 21, "4:3"),
        ];

        for TestCase(h, w, expected) in tests {
            assert_eq!(run(h, w), expected);
        }
    }
}
