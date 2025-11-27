// https://atcoder.jp/contests/abc172/tasks/abc172_b

fn run(s: &str, t: &str) -> usize {
    s.chars()
        .zip(t.chars())
        .filter(|(l, r)| {
            l != r
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, usize);

    #[test]
    fn abc172_b() {
        let tests = [
            TestCase("cupofcoffee", "cupofhottea", 4),
            TestCase("abcde", "bcdea", 5),
            TestCase("apple", "apple", 0),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
