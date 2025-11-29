// https://atcoder.jp/contests/abc175/tasks/abc175_a

use std::cmp::max;

fn run(s: &str) -> usize {
    let mut count = 0;
    let mut ans = 0;

    for c in s.chars() {
        if c == 'R' {
            count += 1;
            ans = ans.max(count);
        } else {
            count = 0;
        }
    }

    ans
}

fn run2(s: &str) -> usize {
    s.chars()
        .fold((0, 0), |(count, ans), c| {
            if c == 'R' {
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

    struct TestCase(&'static str, usize);

    #[test]
    fn abc175_a() {
        let tests = [
            TestCase("RRR", 3),
            TestCase("RRS", 2),
            TestCase("SRR", 2),
            TestCase("SSS", 0),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
