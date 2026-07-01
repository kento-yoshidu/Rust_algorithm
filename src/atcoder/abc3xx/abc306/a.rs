// https://atcoder.jp/contests/abc306/tasks/abc306_a

pub fn run(_n: usize, s: &str) -> String {
    let mut ans = String::new();

    for c in s.chars() {
        ans.push(c);
        ans.push(c);
    }

    ans
}

pub fn run2(_n: usize, s: &str) -> String {
    s.chars().map(|c| {
        format!("{}{}", c, c)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc306_a() {
        let tests = [
            TestCase(8, "beginner", "bbeeggiinnnneerr"),
            TestCase(3, "aaa", "aaaaaa"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
