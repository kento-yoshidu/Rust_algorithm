// https://atcoder.jp/contests/abc399/tasks/abc399_a

fn run(_n: usize, s: &str, t: &str) -> usize {
    s.chars()
        .zip(t.chars())
        .filter(|(s, t)| s != t)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, "abcarc", "agcahc", 2),
            TestCase(7, "atcoder", "contest", 7),
            TestCase(8, "chokudai", "chokudai", 0),
            TestCase(10, "vexknuampx", "vzxikuamlx", 4),
        ];

        for TestCase(n, s, t, expected) in tests {
            assert_eq!(run(n, s, t), expected);
        }
    }
}
