// https://atcoder.jp/contests/abc069/tasks/abc069_b

fn run(s: &str) -> String {
    format!("{}{}{}", &s.chars().nth(0).unwrap(), s.len()-2, &s.chars().last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("internationalization", "i18n"),
            TestCase("smiles", "s4s"),
            TestCase("xyz", "x1z"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
