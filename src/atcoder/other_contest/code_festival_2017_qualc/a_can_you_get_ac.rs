// https://atcoder.jp/contests/code-festival-2017-qualc/tasks/code_festival_2017_qualc_a

use itertools::Itertools;

fn run(s: &str) -> &'static str {
    if s.chars()
        .tuple_windows()
        .any(|(l, r)| {
            l == 'A' && r == 'C'
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
    fn test() {
        let tests = [
            TestCase("BACD" , "Yes"),
            TestCase("ABCD" , "No"),
            TestCase("CABD" , "No"),
            TestCase("ACACA" , "Yes"),
            TestCase("XX" , "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
