// https://atcoder.jp/contests/awc0124/tasks/awc0124_a

use std::cmp::max;

fn run(_n: usize, s: &str) -> usize {
    s.chars()
        .fold((0, 0), |(ans, state), c| {
            match c {
                'L' => {
                    (ans, 0)
                },
                'W' => {
                    (max(ans, state + 1), state + 1)
                },
                _ => unreachable!(),
            }
        })
        .0 as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn awc0124_a() {
        let tests = [
            TestCase(8, "WLWWWLLW", 3),
            TestCase(5, "LLLLL", 0),
            TestCase(20, "WWLLWWWWWLWLWWWLLLWW", 5),
            TestCase(60, "LWWWWLWWWLLWWWWWWLWLWWWWLWWLLLLWWWWWWWLWLWWWWWWWWLLLWWLWWWWW", 8),
            TestCase(1, "W", 1),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
