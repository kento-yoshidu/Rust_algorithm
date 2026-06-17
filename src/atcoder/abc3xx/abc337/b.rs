// https://atcoder.jp/contests/abc337/tasks/abc337_b

fn run(s: &str) -> &'static str {
    let mut t: Vec<char> = s.chars().collect();
    
    t.sort();
    
    if s.chars().collect::<Vec<char>>() == t {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc337_b() {
        let tests = [
            TestCase("AAABBBCCCCCCC", "Yes"),
            TestCase("ACABABCBC", "No"),
            TestCase("A", "Yes"),
            TestCase("ABBBBBBBBBBBBBCCCCCC", "Yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
