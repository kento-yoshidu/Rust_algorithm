// https://atcoder.jp/contests/abc249/tasks/abc249_b

use itertools::Itertools;

pub fn run(s: &str) -> &'static str {
    if !s.chars().all_unique() {
        return "No"
    }

    if s.chars().all(|c| {
        c.is_uppercase()
    }) {
        return "No"
    }

    if s.chars().all(|c| {
        c.is_lowercase()
    }) {
        return "No";
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Yes", run("Aa"));
        assert_eq!("Yes", run("AtCoder"));
        assert_eq!("No", run("atcoder"));
        assert_eq!("No", run("Perfect"));
    }
}
