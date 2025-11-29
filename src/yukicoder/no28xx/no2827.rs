// https://yukicoder.me/problems/no/2827

fn run(_n: usize, s: &str, a: Vec<usize>) -> String {
    a.into_iter()
        .map(|i| s.chars().nth(i-1).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, Vec<usize>, &'static str);

    #[test]
    fn yuki_2827() {
        let tests = [
            TestCase(3, "abc", vec![2, 3, 1], "bca"),
            TestCase(7, "chatgpt", vec![3, 4, 7, 5, 6, 2, 1], "attgphc"),
            TestCase(13, "happyhappycat", vec![6, 2, 4, 8, 10, 11, 12, 9, 3, 5, 1, 7, 13], "happycappyhat"),
        ];

        for TestCase(n, s, a, expected) in tests {
            assert_eq!(run(n, s, a), expected);
        }
    }
}
