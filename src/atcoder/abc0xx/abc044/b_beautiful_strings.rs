// https://atcoder.jp/contests/abc044/tasks/abc044_b

use itertools::Itertools;

fn run(w: &str) -> &'static str {
    let hashmap = w.chars().counts();

    if hashmap.iter().all(|(_, value)| {
        value % 2 == 0
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
            TestCase("abaccaba", "Yes"),
            TestCase("hthth", "No"),
        ];

        for TestCase(w, expected) in tests {
            assert_eq!(run(w), expected);
        }
    }
}
