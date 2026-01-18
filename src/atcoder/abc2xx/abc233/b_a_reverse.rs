// https://atcoder.jp/contests/abc233/tasks/abc233_b

fn run(l: usize, r: usize, s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars[l-1..=r-1].reverse();

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str);

    #[test]
    fn abc233_b() {
        let tests = [
            TestCase(3, 7, "abcdefgh", "abgfedch"),
            TestCase(1, 7, "reviver", "reviver"),
            TestCase(4, 13, "merrychristmas", "meramtsirhcyrs"),
        ];

        for TestCase(l, r, s, expected) in tests {
            assert_eq!(run(l, r, s), expected);
        }
    }
}
