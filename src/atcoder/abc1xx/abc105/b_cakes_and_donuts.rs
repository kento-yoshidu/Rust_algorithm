// https://atcoder.jp/contests/abc105/tasks/abc105_b

fn run(n: usize) -> &'static str {
    for i in 0..=n/4 {
        for j in 0..=n/7 {
            if i*4 + j*7 == n {
                return "Yes";
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(11, "Yes"),
            TestCase(40, "Yes"),
            TestCase(3, "No"),
            TestCase(13, "No"),
            TestCase(41, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
