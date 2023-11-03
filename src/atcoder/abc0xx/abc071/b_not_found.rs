// https://atcoder.jp/contests/abc071/tasks/abc071_b

use itertools::Itertools;

pub fn run(s: String) -> String {
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

    #[test]
    fn test() {
        assert_eq!(String::from("b"), run(String::from("atcoderregularcontest")));
        assert_eq!(String::from("None"), run(String::from("abcdefghijklmnopqrstuvwxyz")));
        assert_eq!(String::from("d"), run(String::from("fajsonlslfepbjtsaayxbymeskptcumtwrmkkinjxnnucagfrg")));
    }
}
