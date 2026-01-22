// https://atcoder.jp/contests/abc237/tasks/abc237_a

fn run(n: isize) -> &'static str {
    if -2_isize.pow(31) <= n && n < 2_isize.pow(31) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, &'static str);

    #[test]
    fn abc237_a() {
        let tests = [
            TestCase(5, "Yes"),
            TestCase(-9876543210, "No"),
            TestCase(483597848400000, "No"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
