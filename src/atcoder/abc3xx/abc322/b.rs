// https://atcoder.jp/contests/abc322/tasks/abc322_b

fn run(n: usize, m: usize, s: &str, t: &str) -> usize {
    if (s == t) || (&t[0..n] == s && &t[m-n..] == s) {
        0
    } else if &t[0..n] == s  {
        1
    } else if &t[m-n..] == s {
        2
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str, usize);

    #[test]
    fn abc322_b() {
        let tests = [
            TestCase(3, 7, "abc", "abcdefg", 1),
            TestCase(3, 4, "abc", "aabc", 2),
            TestCase(3, 3, "abc", "xyz", 3),
            TestCase(3, 3, "aaa", "aaa", 0),
        ];

        for TestCase(n, m, s, t, expected) in tests {
            assert_eq!(run(n, m, s, t), expected);
        }
    }
}
