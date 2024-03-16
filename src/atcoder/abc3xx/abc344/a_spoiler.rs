// https://atcoder.jp/contests/abc344/tasks/abc344_a

fn run(s: &'static str) -> String {
    let vec: Vec<&'static str> = s.split('|').collect();

    format!("{}{}", vec[0], vec[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("atcoder|beginner|contest", "atcodercontest"),
            TestCase("|spoiler|", ""),
            TestCase("||xyz", "xyz"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
