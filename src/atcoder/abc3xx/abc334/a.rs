// https://atcoder.jp/contests/abc334/tasks/abc334_a

fn run(b: usize, g: usize) -> &'static str {
    if b > g {
        "Bat"
    } else {
        "Glove"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc334_a() {
        let tests = [
            TestCase(300, 100, "Bat"),
            TestCase(334, 343, "Glove"),
        ];

        for TestCase(b, g, expected) in tests {
            assert_eq!(run(b, g), expected);
        }
    }
}
