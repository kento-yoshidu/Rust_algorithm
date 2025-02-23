// https://atcoder.jp/contests/abc017/tasks/abc017_2

fn run(s: &str) -> &'static str {
    if s.chars()
        .any(|c| {
            !['c', 'h', 'o', 'k', 'u'].contains(&c)
        }) {
            return "NO";
        }

    if s.chars().last().unwrap() == 'c' {
        return "NO";
    }

    if s.chars().nth(0).unwrap() == 'h' {
        return "NO";
    }

    if s.chars().collect::<Vec<char>>()
        .windows(2)
        .any(|t| {
            (t[0] == 'c' && t[1] != 'h') || (t[0] != 'c' && t[1] == 'h')
        }) {
            return "NO";
        } else {
            return "YES";
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("chokuou", "YES"),
            TestCase("kuccho", "NO"),
            TestCase("atcoder", "NO"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
