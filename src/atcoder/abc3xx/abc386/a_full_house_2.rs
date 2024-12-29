// https://atcoder.jp/contests/abc386/tasks/abc386_a

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize, d: usize) -> &'static str {
    if [a, b, c, d]
        .into_iter()
        .sorted()
        .dedup()
        .count() == 2 {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests  = [
            TestCase(7, 7, 7, 1, "Yes"),
            TestCase(13, 12, 11, 10, "No"),
            TestCase(3, 3, 5, 5, "Yes"),
            TestCase(8, 8, 8, 8, "No"),
            TestCase(1, 3, 4, 1, "No"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
