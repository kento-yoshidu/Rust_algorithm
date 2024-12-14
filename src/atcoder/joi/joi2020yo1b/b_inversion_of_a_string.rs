// https://atcoder.jp/contests/joi2020yo1b/tasks/joi2020_yo1b_b

fn run(_n: usize, a: usize, b: usize, s: &str) -> String {
    format!("{}{}{}", &s[0..a-1], &s[a-1..b].chars().rev().collect::<String>(), &s[b..])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 3, 7, "JOIjoiJoIj", "JOJiojIoIj"),
            TestCase(9, 6, 6, "abcdefghi", "abcdefghi"),
        ];

        for TestCase(n, a, b, s, expected) in tests {
            assert_eq!(run(n, a, b, s), expected);
        }
    }
}
