// https://atcoder.jp/contests/abc071/tasks/abc071_b

use itertools::Itertools;

fn run(s: &str) -> String {
    ('a'..='z').find(|c| {
        !s.chars().contains(c)
    })
    .map(|c| {
        c.to_string()
    }).unwrap_or(String::from("None"))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("atcoderregularcontest", "b"),
            TestCase("abcdefghijklmnopqrstuvwxyz", "None"),
            TestCase("fajsonlslfepbjtsaayxbymeskptcumtwrmkkinjxnnucagfrg", "d"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
