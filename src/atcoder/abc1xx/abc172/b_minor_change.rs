// https://atcoder.jp/contests/abc172/tasks/abc172_b

pub fn run(s: &str, t: &str) -> usize {
    s.chars()
        .zip(t.chars())
        .filter(|(l, r)| {
            l != r
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run("cupofcoffee", "cupofhottea"));
        assert_eq!(5, run("abcde", "bcdea"));
        assert_eq!(0, run("apple", "apple"));
    }
}
