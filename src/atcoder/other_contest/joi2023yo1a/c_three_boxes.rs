// https://atcoder.jp/contests/joi2023yo1a/tasks/joi2023_yo1a_c

use std::cmp::{min, max};

fn run(_n: usize, s: &str) -> usize {
    let mut pos = 1;

    s.chars()
        .map(|c| {
            match c {
                'L' => {
                    pos = max(1, pos-1);
                    0
                },
                'R' => {
                    pos = min(3, pos+1);
                    if pos == 3 {
                        1
                    } else {
                        0
                    }
                },
                _ => unreachable!(),
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "LRRR", 2),
            TestCase(3, "LRL", 0),
            TestCase(7, "LRLRRRL", 2),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
