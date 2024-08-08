// https://atcoder.jp/contests/arc177/tasks/arc177_b

pub fn run(n: usize, s: &str) -> (usize, String) {
    let mut str = String::new();

    for i in (0..n).rev() {
        if s.chars().nth(i).unwrap() == '1' {
            for _ in 0..=i {
                str.push('A');
            }
            for _ in 0..i {
                str.push('A');
            }
        }
    }

    (str.len(), str)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, (usize, String));

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "01100", (8, "AAAAAAAA".to_string())),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
