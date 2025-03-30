// https://atcoder.jp/contests/abc382/tasks/abc382_a

fn run(n: usize, d: usize, s: &str) -> usize {
    let count = s.chars()
        .filter(|c| *c == '@')
        .count();

    n - count + d
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, ".@@.@", 4),
            TestCase(3, 3, "@@@", 3),
            TestCase(10, 4, "@@@.@@.@@.", 7),
        ];

        for TestCase(n, d, s, expected) in tests {
            assert_eq!(run(n, d, s), expected);
        }
    }
}
