// https://atcoder.jp/contests/abc197/tasks/abc197_a

fn run(s: &str) -> String {
    let mut c = s.chars();

    let mut ans = String::new();

    let c1 = c.nth(0).unwrap();
    let c2 = c.nth(0).unwrap();
    let c3 = c.nth(0).unwrap();

    ans.push(c2);
    ans.push(c3);
    ans.push(c1);

    ans
}

fn run2(s: &str) -> String {
    format!("{}{}", &s[1..], &s[..1])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("abc", "bca"),
            TestCase("aab", "aba"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
