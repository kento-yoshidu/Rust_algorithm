// https://atcoder.jp/contests/abc016/tasks/abc016_2

fn run(s: &str) -> usize {
    let a = &s.chars().position(|c| {
        c == 'A'
    }).unwrap();

    let z = &s.chars().rev().position(|c| {
        c == 'Z'
    }).unwrap();

    s.len() - z - a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("QWERTYASDFZXCV", 5),
            TestCase("ZABCZ", 4),
            TestCase("HASFJGHOGAKZZFEGA", 12),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
