// https://atcoder.jp/contests/abc364/tasks/abc364_a

pub fn run(n: usize, s: Vec<&str>) -> &'static str {
    if n == 1 {
        return "Yes";
    }

    if (0..n-2)
        .any(|i| {
            s[i] == "sweet" && s[i+1] == "sweet"
        }) {
            "No"
        } else {
            "Yes"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec!["salty", "sweet", "salty", "salty", "sweet"], "Yes"),
            TestCase(4, vec!["sweet", "salty", "sweet", "sweet"], "Yes"),
            TestCase(6, vec!["salty", "sweet", "sweet", "salty", "sweet", "sweet"], "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
