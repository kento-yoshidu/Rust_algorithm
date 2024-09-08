// https://atcoder.jp/contests/abc370/tasks/abc370_a

fn run(l: usize, r: usize) -> &'static str {
    if l == r {
        "Invalid"
    } else if l == 1 {
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
    fn test() {
        let tests = [
            TestCase(1, 0, "Yes"),
            TestCase(1, 1, "Invalid"),
            TestCase(0, 1, "No"),
            TestCase(0, 0, "Invalid"),
        ];

        for TestCase(l, r, expected) in tests {
            assert_eq!(run(l, r), expected);
        }
    }
}
