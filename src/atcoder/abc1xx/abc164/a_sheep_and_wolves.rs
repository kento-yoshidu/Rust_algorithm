// https://atcoder.jp/contests/abc164/tasks/abc164_a

fn run(s: usize, w: usize) -> &'static str {
    if w >= s {
        "unsafe"
    } else {
        "safe"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc164_a() {
        let tests = [
            TestCase(4, 5, "unsafe"),
            TestCase(100, 2, "safe"),
            TestCase(10, 10, "unsafe"),
        ];

        for TestCase(s, w, expected) in tests {
            assert_eq!(run(s, w), expected);
        }
    }
}
