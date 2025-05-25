// https://atcoder.jp/contests/abc104/tasks/abc104_a

fn run(r: usize) -> &'static str {
    if r < 1200 {
        "ABC"
    } else if r < 2800 {
        "ARC"
    } else {
        "AGC"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc104_a() {
        let tests = [
            TestCase(1199, "ABC"),
            TestCase(1200, "ARC"),
            TestCase(4208, "AGC"),
        ];

        for TestCase(r, expected) in tests {
            assert_eq!(run(r), expected);
        }
    }
}
