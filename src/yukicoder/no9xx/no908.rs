// https://yukicoder.me/problems/no/908

pub fn run(s: &str) -> &'static str {
    if s.chars()
        .enumerate()
        .all(|(i, c)| {
            (i % 2 == 0) == (c != ' ')
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn yuki_908() {
        let tests = [
            TestCase("u s h i t a p u n i c h i a k u n", "Yes"),
            TestCase("van emde boas tree", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
