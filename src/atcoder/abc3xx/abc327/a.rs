// https://atcoder.jp/contests/abc327/tasks/abc327_a

fn run(_n: usize, s: &str) -> &'static str {
    let chars: Vec<char> = s.chars().collect();

    if chars.windows(2)
        .any(|t| {
            t[0] == 'a' && t[1] == 'b' || t[0] == 'b' && t[1] == 'a'
            })
        {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc327_a() {
        let tests = [
            TestCase(3, "abc", "Yes"),
            TestCase(3, "ba", "Yes"),
            TestCase(7, "atcoder", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
