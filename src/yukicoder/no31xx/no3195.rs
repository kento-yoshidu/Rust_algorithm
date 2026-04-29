// https://yukicoder.me/problems/no/3195

fn run(a: &str, b: &str, c: &str) -> String {
    [a, b, c].into_iter()
        .flat_map(|str| {
            str.chars().nth(0).unwrap().to_uppercase()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str, &'static str);

    #[test]
    fn yuki_3195() {
        let tests = [
            TestCase("three", "letter", "acronym", "TLA"),
            TestCase("in", "for", "loop", "IFL"),
            TestCase("a", "b", "c", "ABC"),
        ];

        for TestCase(a, b, c , expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
