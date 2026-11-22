// https://atcoder.jp/contests/abc060/tasks/abc060_a

fn run(a: isize, b: isize, c: isize) -> &'static str {
    if a <= c && c <= b {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, &'static str);

    #[test]
    fn abc061_a() {
        let tests = [
            TestCase(1, 3, 2, "Yes"),
            TestCase(1, 3, 2, "Yes"),
            TestCase(6, 5, 4, "No"),
            TestCase(2, 2, 2, "Yes"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
