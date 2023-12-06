// https://atcoder.jp/contests/abc280/tasks/abc280_c

fn run(s: &str, t: &str) -> usize {
    s.chars()
        .zip(t.chars())
        .position(|(l, r)| {
            l != r
        })
        .unwrap_or(s.len()) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run("atcoder", "atcorder"));
        assert_eq!(5, run("million", "milllion"));
        assert_eq!(3, run("vvwvw", "vvvwvw"));
    }
}
