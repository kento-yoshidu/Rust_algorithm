// https://atcoder.jp/contests/abc236/tasks/abc236_a

fn run(s: &str, a: usize, b: usize) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars.swap(a-1, b-1);

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, usize, &'static str);

    #[test]
    fn abc236_a() {
        let tests = [
            TestCase("chokudai", 3, 5, "chukodai"),
            TestCase("aa", 1, 2, "aa"),
            TestCase("aaaabbbb", 1, 8, "baaabbba"),
        ];

        for TestCase(s, a, b, expected) in tests {
            assert_eq!(run(s, a, b), expected);
        }
    }
}
