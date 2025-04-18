// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_b

fn run(s: &str) -> (usize, usize) {
    let chars: Vec<char> = s.chars().collect();

    chars
        .windows(3)
        .fold((0, 0), |acc, c| {
            if c == ['J', 'O', 'I'] {
                (acc.0+1, acc.1)
            } else if c == ['I', 'O', 'I'] {
                (acc.0, acc.1+1)
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, (usize, usize));

    #[test]
    fn test() {
        let tests = [
            TestCase("JOIJOI", (2, 0)),
            TestCase("JOIOIOIOI", (1, 3)),
            TestCase("JOIOIJOINXNXJIOIOIOJ", (2, 3)),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
