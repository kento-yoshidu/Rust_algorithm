// https://yukicoder.me/problems/no/400/

fn run(s: &str) -> String {
    s.chars()
        .rev()
        .map(|c| {
            match c {
                '>' => '<',
                '<' => '>',
                _ => unreachable!()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn yuki_400() {
        let tests = [
            TestCase("<<<", ">>>"),
            TestCase("<>>", "<<>"),
            TestCase(">>><<<", ">>><<<"),
            TestCase("><<><<<><><", "><><>>><>><"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
