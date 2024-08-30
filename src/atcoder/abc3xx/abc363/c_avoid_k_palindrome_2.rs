// https://atcoder.jp/contests/abc363/tasks/abc363_c

use itertools::Itertools;
use std::collections::HashSet;

fn check(chars: &[char]) -> bool {
    chars.iter().eq(chars.iter().rev())
}

fn run(n: usize, k: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    let mut hash_set: HashSet<Vec<char>> = HashSet::new();

    for str in chars.into_iter().permutations(n) {
        hash_set.insert(str);
    }

    let mut ans = 0;

    'outer: for str in hash_set.into_iter() {
        // k文字の部分文字列生成
        for arr in str.windows(k) {
            // 部分文字列に一つでも回文があったら次の文字に進む
            if check(arr) == true {
                continue 'outer;
            }
        }

        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, "aab", 1),
            TestCase(5, 3, "zzyyx", 16),
            TestCase(10, 5, "abcwxyzyxw", 440640),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
