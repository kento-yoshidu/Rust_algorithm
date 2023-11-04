// https://atcoder.jp/contests/abc175/tasks/abc175_a

pub fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.windows(2)
        .filter(|t| {
            t[0] == 'R' && t[1] == 'R'
        })
        .count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run("RRR"));
        assert_eq!(2, run("RRS"));
        assert_eq!(2, run("SRR"));
        assert_eq!(0, run("SSS"));
    }
}
