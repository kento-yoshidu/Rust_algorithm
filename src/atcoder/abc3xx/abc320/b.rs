// https://atcoder.jp/contests/abc320/tasks/abc320_b

fn check(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn run(s: &str) -> usize {
    let mut ans = 1;

    for i in 0..s.len() {
        for j in i+1..s.len() {
            if check(&s[i..=j]) {
                ans = ans.max(j+1 - i);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc320_b() {
        let tests = [
            TestCase("TOYOTA", 5),
            TestCase("ABCDEFG", 1),
            TestCase("AAAAAAAAAA", 10),
            TestCase("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", 100),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
