// https://atcoder.jp/contests/abc360/tasks/abc360_a

use itertools::Itertools;

fn run(s: &str) -> &'static str {
    let r = s.chars().find_position(|c| *c == 'R').unwrap();
    let m = s.chars().find_position(|c| *c == 'M').unwrap();

    if r < m {
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
            TestCase("RSM", "Yes"),
            TestCase("SMR", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
