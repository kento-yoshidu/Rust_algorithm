// https://atcoder.jp/contests/abc018/tasks/abc018_2

pub fn run(s: &str, _n: usize, l: Vec<(usize, usize)>) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    for (l, r) in l {
        chars[l-1..=r-1].reverse();
    }

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("debacf"), run("abcdef", 2, vec![(3, 5), (1, 4)]));
        assert_eq!(String::from("atcoder"), run("redcoat", 3, vec![(1, 7), (1, 2), (3, 4)]));
    }
}
