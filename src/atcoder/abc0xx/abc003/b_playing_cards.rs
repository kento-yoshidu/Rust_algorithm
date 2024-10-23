// https://atcoder.jp/contests/abc003/tasks/abc003_2

fn run(s: &str, t: &str) -> &'static str {
    let arr = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

    if s.chars()
        .zip(t.chars())
        .all(|t| {
            if t.0 == t.1 {
                true
            } else {
                if t.0 == '@' && t.1 == '@' {
                    true
                } else if t.0 != '@' && t.1 != '@' {
                    false
                } else if t.0 == '@' && arr.contains(&t.1) {
                    true
                } else if t.1 == '@' && arr.contains(&t.0) {
                    true
                } else {
                    false
                }
            }
        }) {
            "You can win"
        } else {
            "You will lose"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("ch@ku@ai", "choku@@i", "You can win"),
            TestCase("aoki", "@ok@", "You will lose"),
            TestCase("arc", "abc", "You will lose"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
