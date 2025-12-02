// https://yukicoder.me/problems/no/3035

fn run(s: &str) -> String {
    let pos = s.chars().position(|c| !c.is_ascii_alphabetic()).unwrap();

    format!("{}{}", &s[0..pos].chars().rev().collect::<String>(), &s[pos..])
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase(&'static str, &'static str);

    #[test]
    fn yuki_3035() {
        let tests = [
            TestCase("eiram1234", "marie1234"),
            TestCase("abc352", "cba352"),
            TestCase("happynewyear2025", "raeywenyppah2025"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
