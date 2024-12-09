// https://atcoder.jp/contests/joi2022yo1b/tasks/joi2022_yo1b_c

fn run(n: usize, s: &str) -> Vec<char> {
    let chars: Vec<char> = s.chars().collect();

    (1..n)
        .filter(|i| {
            chars[*i] == 'J'
        })
        .map(|i| {
            chars[i-1]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, Vec<char>);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, "IOJOIJ", vec!['O', 'I']),
            TestCase(4, "JJOI", vec!['J']),
            TestCase(7, "IOJOJOJ", vec!['O', 'O', 'O']),
            TestCase(5, "JJJJJ", vec!['J', 'J', 'J', 'J']),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
