// https://atcoder.jp/contests/abc374/tasks/abc374_b

use itertools::{Itertools, EitherOrBoth};

fn run(s: &str, t: &str) -> usize {
    s.chars()
        .zip_longest(t.chars())
        .position(|ab|  match ab{
            EitherOrBoth::Both(a, b) => {
                    a != b
            },
            // 長さが違ったらpositionを返す
            EitherOrBoth::Left(_) | EitherOrBoth::Right(_) => true
        })
        .map_or(0, |pos| pos+1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("abcde", "abedc", 3),
            TestCase("abcde", "abcdefg", 6),
            TestCase("keyence", "keyence", 0),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
