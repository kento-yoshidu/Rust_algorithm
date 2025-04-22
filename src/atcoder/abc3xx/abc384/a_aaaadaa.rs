// https://atcoder.jp/contests/abc384/tasks/abc384_a

fn run(_n: usize, c1: char, c2: char, s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == c1 {
                c1
            } else {
                c2
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, char, char, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 'b', 'g', "abc", "gbg"),
            TestCase(1, 's', 'h', "s", "s"),
            TestCase(7, 'd', 'a', "atcoder", "aaaadaa"),
            TestCase(10, 'b', 'a', "acaabcabba", "aaaabaabba"),
        ];

        for TestCase(n, c1, c2, s, expected) in tests {
            assert_eq!(run(n, c1, c2, s), expected);
        }
    }
}
