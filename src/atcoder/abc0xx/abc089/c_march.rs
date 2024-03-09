// https://atcoder.jp/contests/abc089/tasks/abc089_c

use itertools::Itertools;

fn run(_n: usize, s: Vec<&str>) -> usize {
    let mut vec = vec![0; 5];

    for c in s {
        match c.chars().nth(0).unwrap() {
            'M' => vec[0] += 1,
            'A' => vec[1] += 1,
            'R' => vec[2] += 1,
            'C' => vec[3] += 1,
            'H' => vec[4] += 1,
            _ => (),
        }
    }

    let mut ans = 0;

    for (a, b, c) in (0..5).tuple_combinations() {
        ans += vec[a] * vec[b] * vec[c];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec!["MASHIKE", "RUMOI", "OBIRA", "HABORO", "HOROKANAI"], 2),
            TestCase(4, vec!["ZZ", "ZZZ", "Z", "ZZZZZZZZZZ"], 0),
            TestCase(5, vec!["CHOKUDAI", "RNG", "MAKOTO", "AOKI", "RINGO"], 7),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
