// https://atcoder.jp/contests/abl/tasks/abl_b

fn run(a: usize, b: usize, c: usize, d: usize) -> &'static str {
    if a < c && c <= b || c < a && a <= d {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 30, 20, 40, "Yes"),
            TestCase(10, 20, 30, 40, "No"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
