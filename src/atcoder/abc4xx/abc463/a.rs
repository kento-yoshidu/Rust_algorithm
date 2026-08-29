// https://atcoder.jp/contests/abc463/tasks/abc463_a

fn run(x: usize, y: usize) -> &'static str {
    if x * 9 == y * 16 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc463_a() {
        let tests = [
            TestCase(800, 450, "Yes"),
            TestCase(234, 108, "No"),
            TestCase(108, 192, "No"),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
