// https://atcoder.jp/contests/abc175/tasks/abc175_a

use std::cmp::max;

pub fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    let mut count = 0;
    let mut ans = 0;

    chars.iter()
        .for_each(|c| {
            if *c == 'R' {
                count += 1;
                ans = ans.max(count);
            } else {
                count = 0;
            }
        });

    ans
}

pub fn run2(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .fold((0, 0), |(count, ans), c| {
            if *c == 'R' {
                (count+1, max(ans, count+1))
            } else {
                (0, ans)
            }
        })
        .1
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

    #[test]
    fn test2() {
        assert_eq!(3, run2("RRR"));
        assert_eq!(2, run2("RRS"));
        assert_eq!(2, run2("SRR"));
        assert_eq!(0, run2("SSS"));
    }
}
