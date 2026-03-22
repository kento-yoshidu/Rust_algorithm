// https://yukicoder.me/problems/no/3419

fn run(_n: usize, s: &str, p: Vec<usize>) -> String {
    p.into_iter()
        .map(|i| s.chars().nth(i-1).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, Vec<usize>, &'static str);

    #[test]
    fn yuki_3419() {
        let tests = [
            TestCase(8, "anagrams", vec![3, 5, 8, 7, 1, 4, 2, 6], "arsmagna"),
            TestCase(8, "triangle", vec![3, 5, 1, 8, 6, 2, 4, 7], "integral"),
        ];

        for TestCase(n, s, p, expected) in tests {
            assert_eq!(run(n, s, p), expected);
        }
    }
}
