// https://atcoder.jp/contests/abc311/tasks/abc311_a

fn run(_n: usize, s: String) -> usize {
    let mut ans = 0;

    for abc in ['A', 'B', 'C'] {
        ans = ans.max(s.chars().position(|c| c == abc).unwrap())
    }

    ans + 1
}

fn run2(_n: usize, s: String) -> usize {
    ['A', 'B', 'C'].iter().map(|&abc| {
        s.chars().position(|c| c == abc).unwrap()
    }).max().unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(5, String::from("ACABB")));
        assert_eq!(3, run(4, String::from("CABC")));
        assert_eq!(17, run(30, String::from("AABABBBABABBABABCABACAABCBACCA")));
    }

    #[test]
    fn test2() {
        assert_eq!(4, run2(5, String::from("ACABB")));
        assert_eq!(3, run2(4, String::from("CABC")));
        assert_eq!(17, run2(30, String::from("AABABBBABABBABABCABACAABCBACCA")));
    }
}
