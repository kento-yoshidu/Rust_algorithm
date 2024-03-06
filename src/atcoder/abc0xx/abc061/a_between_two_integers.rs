// https://atcoder.jp/contests/abc060/tasks/abc060_a

pub fn run(a: i32, b: i32, c: i32) -> &'static str {
    if a <= c && c <= b {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, i32, &'static str);

    #[test]
    fn test() {
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
