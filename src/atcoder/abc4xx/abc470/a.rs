// https://atcoder.jp/contests/abc470/tasks/abc470_a

fn run(n: usize) -> Vec<String> {
    (1..=n)
        .map(|n| {
            if n % 3 == 0 {
                "Fizz".to_string()
            } else {
                n.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>);

    #[test]
    fn abc470_a() {
        let tests = [
            TestCase(4, vec!["1", "2", "Fizz", "4"]),
            TestCase(10, vec!["1", "2", "Fizz", "4", "5", "Fizz", "7", "8", "Fizz", "10"]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
