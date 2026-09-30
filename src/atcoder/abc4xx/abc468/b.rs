// https://atcoder.jp/contests/abc468/tasks/abc468_b

use std::cmp::{min, max};

fn run(m: isize, d: isize, s: &str) -> usize {
    let s: Vec<char> = s.chars().collect();

    let mut ans = 0;

    'outer: for i in 0..m {
        if s[i as usize] == 'G' {
            continue;
        }

        let i = i as isize;

        for j in max(0, i-d)..=min(m-1, i+d) {
            if s[j as usize] == 'G' {
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

    struct TestCase(isize, isize, &'static str, usize);

    #[test]
    fn abc468_b() {
        let tests = [
            TestCase(7, 1, ".G...GG", 1),
            TestCase(6, 5, "......", 6),
            TestCase(21, 2, "....G...GG.....G.....", 6),
        ];

        for TestCase(m, d, s, expected) in tests {
            assert_eq!(run(m, d, s), expected);
        }
    }
}
