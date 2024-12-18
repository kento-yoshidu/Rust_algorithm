// https://atcoder.jp/contests/abc058/tasks/abc058_a

fn run(a: i32, b: i32, c: i32) -> &'static str {
    if b - a == c - b {
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
            TestCase(2, 4, 6, "Yes"),
            TestCase(2, 5, 6, "No"),
            TestCase(3, 2, 1, "Yes"),
        ];

        for TestCase(a, b, c , expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
