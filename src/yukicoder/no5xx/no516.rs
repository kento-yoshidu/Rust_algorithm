// https://yukicoder.me/problems/no/516

fn run(s1: &str, s2: &str, s3: &str) -> &'static str {
    let red_count = [s1, s2, s3].iter().filter(|&&s| s == "RED").count();

    if red_count >= 2 {
        "RED"
    } else {
        "BLUE"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str, &'static str);

    #[test]
    fn yuki_516() {
        let tests = [
            TestCase("RED", "RED", "BLUE", "RED"),
            TestCase("BLUE", "BLUE", "BLUE", "BLUE"),
        ];

        for TestCase(s1, s2, s3, expected) in tests {
            assert_eq!(run(s1, s2, s3), expected);
        }
    }
}
