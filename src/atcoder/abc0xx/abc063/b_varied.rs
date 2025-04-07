// https://atcoder.jp/contests/abc063/tasks/abc063_b

use itertools::Itertools;

fn run(s: String) -> &'static str {
    if s.chars().all_unique() {
        "yes"
    } else {
        "no"
    }
}

fn run2(s: String) -> &'static str {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();
    chars.dedup();

    if s.len() == chars.len() {
        "yes"
    } else {
        "no"
    }
}

fn run3(s: String) -> &'static str {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();

    if chars.windows(2).all(|v| {
        v[0] != v[1]
    }) {
        "yes"
    } else {
        "no"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("uncopyrightable", "yes"),
            TestCase("different", "no"),
            TestCase("no", "yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s.to_string()), expected);
            assert_eq!(run2(s.to_string()), expected);
            assert_eq!(run3(s.to_string()), expected);
        }
    }
}
