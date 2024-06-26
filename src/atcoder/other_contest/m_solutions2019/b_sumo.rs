// https://atcoder.jp/contests/m-solutions2019/tasks/m_solutions2019_b

fn run(s: &str) -> &'static str {
    let count = s.chars()
        .filter(|c| *c == 'o')
        .count();

    if count + (15 - s.len()) >= 8 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("oxoxoxoxoxoxox", "YES"),
            TestCase("xxxxxxxx", "NO"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
