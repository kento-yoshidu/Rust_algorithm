// https://atcoder.jp/contests/joi2024yo1c/tasks/joi2024_yo1c_c

use itertools::Itertools;

fn run(_n: usize, s: &str) -> &'static str {
    if s.chars().all_equal() {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "bbbb", "Yes"),
            TestCase(7, "pppdppp", "No"),
            TestCase(2, "xx", "Yes"),
            TestCase(9, "joijoijoi", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
