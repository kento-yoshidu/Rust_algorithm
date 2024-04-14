// https://atcoder.jp/contests/abc348/tasks/abc348_a

fn run(n: usize) -> String {
    (1..=n)
        .map(|i| {
            if i % 3 == 0 {
                'x'
            } else {
                'o'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, "ooxooxo"),
            TestCase(9, "ooxooxoox"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
