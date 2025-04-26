// https://atcoder.jp/contests/joi2020yo1a/tasks/joi2020_yo1a_b

fn run(_n: usize, s: &str) -> usize {
    s.chars()
        .filter(|c| {
            ['a', 'i', 'u', 'e', 'o'].contains(c)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, "joiyosen", 4),
            TestCase(6, "bitaro", 3),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
