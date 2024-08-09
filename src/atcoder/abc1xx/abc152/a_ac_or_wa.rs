// https://atcoder.jp/contests/abc152/tasks/abc152_a

pub fn run(n: usize, m: usize) -> &'static str {
    if n == m {
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
            TestCase(3, 3, "Yes"),
            TestCase(3, 2, "No"),
            TestCase(1, 1, "Yes"),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
