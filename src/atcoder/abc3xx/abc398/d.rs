// https://atcoder.jp/contests/abc398/tasks/abc398_d

use std::collections::HashSet;

fn run(_n: usize, r: isize, c: isize, s: &str) -> String {
    let mut takahashi = (r, c);
    let mut takibi = (0, 0);

    let mut hash_set = HashSet::new();
    hash_set.insert((0, 0));

    let mut ans = Vec::new();

    for c in s.chars() {
        match c {
            'N' => {
                takahashi.0 += 1;
                takibi.0 += 1;
            },
            'S' => {
                takahashi.0 -= 1;
                takibi.0 -= 1;
            },
            'W' => {
                takahashi.1 += 1;
                takibi.1 += 1;
            },
            'E' => {
                takahashi.1 -= 1;
                takibi.1 -= 1;
            },
            _ => unreachable!(),
        }

        hash_set.insert(takibi);

        if hash_set.contains(&takahashi) {
            ans.push('1');
        } else {
            ans.push('0');
        }
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, isize, &'static str, &'static str);

    #[test]
    fn abc398_d() {
        let tests = [
            TestCase(10, 1, 2, "NEESESWEES", "0001101011"),
            TestCase(20, -1, -2, "WWNNWSWEWNSWWENSNWWN", "00100111111000101111"),
        ];

        for TestCase(n, r, c, s, expected) in tests {
            assert_eq!(run(n, r, c, s), expected);
        }
    }
}
