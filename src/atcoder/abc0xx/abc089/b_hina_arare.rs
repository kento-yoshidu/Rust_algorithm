// https://atcoder.jp/contests/abc089/tasks/abc089_b

fn run(_n: usize, s: &str) -> &'static str {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();
    chars.dedup();

    match chars.len() {
        3 => "Three",
        4 => "Four",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, "GWYPYW", "Four"),
            TestCase(9, "GWWGPWPGG", "Three"),
            TestCase(8, "PYWGYWYY", "Four"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
