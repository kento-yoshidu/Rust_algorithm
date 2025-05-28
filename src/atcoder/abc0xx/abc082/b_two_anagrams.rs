// https://atcoder.jp/contests/abc082/tasks/abc082_b

fn run(s: &str, t: &str) -> &'static str {
    let mut ss: Vec<char> = s.chars().collect();
    ss.sort();

    let mut tt: Vec<char> = t.chars().collect();
    tt.sort();
    tt.reverse();

    if ss < tt {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct  TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("yx", "axy", "Yes"),
            TestCase("ratcode", "atlas", "Yes"),
            TestCase("cd", "abc", "No"),
            TestCase("w", "ww", "Yes"),
            TestCase("zzz", "zzz", "No"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
