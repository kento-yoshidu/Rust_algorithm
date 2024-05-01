// https://atcoder.jp/contests/caddi2018b/tasks/caddi2018b_a

fn run(n: &str) -> usize {
    n.chars()
        .filter(|c| {
            *c == '2'
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("1222", 3),
            TestCase("3456", 0),
            TestCase("9592", 1),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
