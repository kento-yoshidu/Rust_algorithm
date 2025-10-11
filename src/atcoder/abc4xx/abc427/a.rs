// https://atcoder.jp/contests/abc427/tasks/abc427_a

fn run(s: &str) -> String {
    format!("{}{}", &s[0..s.len()/2], &s[s.len()/2+1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc427_a() {
        let tests = [
            TestCase("ABCDE", "ABDE"),
            TestCase("OOO", "OO"),
            TestCase("ATCODER", "ATCDER"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
