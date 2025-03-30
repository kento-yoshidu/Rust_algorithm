// https://atcoder.jp/contests/abc097/tasks/abc097_a

fn run(a: isize, b: isize, c: isize, d: isize) -> &'static str {
    if  (a - c).abs() <= d || ((a - b).abs() <= d && (b - c).abs() <= d) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, &'static str);

    #[test]

    fn test() {
        let tests = [
            TestCase(4, 7, 9, 3, "Yes"),
            TestCase(100, 10, 1, 2, "No"),
            TestCase(10, 10, 10, 1, "Yes"),
            TestCase(1, 100, 2, 10, "Yes"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
