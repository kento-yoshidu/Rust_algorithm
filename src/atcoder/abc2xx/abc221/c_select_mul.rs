// https://atcoder.jp/contests/abc221/tasks/abc221_c

use itertools::Itertools;

pub fn run(n: usize) -> usize {
    let str = n.to_string();

    let mut ans = 0;

    for chars in str.chars().permutations(str.len()) {
        for j in 1..str.len() {
            let l_chars = &chars[0..j];
            let r_chars = &chars[j..];

            if l_chars[0] == '0' || r_chars[0] == '0' {
                continue;
            }

            let l = l_chars.iter()
                .map(|&c| c.to_digit(10).unwrap() as usize)
                .fold(0, |acc, x| acc * 10 + x);

            let r = r_chars.iter()
                .map(|&c| c.to_digit(10).unwrap() as usize)
                .fold(0, |acc, x| acc * 10 + x);

            ans = ans.max(l*r);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(123, 63),
            TestCase(1010, 100),
            TestCase(998244353, 939337176),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
