// https://atcoder.jp/contests/abc358/tasks/abc358_a

fn run(s: &str, t: &str) -> &'static str {
    if s == "AtCoder" && t == "Land" {
        return "Yes";
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("AtCoder", "Land", "Yes"),
            TestCase("CodeQUEEN", "Land", "No"),
            TestCase("aTcodeR", "lANd", "No"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
