// https://atcoder.jp/contests/abc291/tasks/abc291_c

use std::collections::HashSet;

fn run(_n: usize, s: &str) -> &'static str {
    let mut pos: (isize, isize) = (0, 0);

    let mut hash_set = HashSet::from([pos]);

    for c in s.chars() {
        let new_pos = match c {
            'U' => {
                (pos.0-1, pos.1)
            },
            'R' => {
                (pos.0, pos.1+1)
            },
            'D' => {
                (pos.0+1, pos.1)
            },
            'L' => {
                (pos.0, pos.1-1)
            },
            _ => unreachable!(),
        };

        if hash_set.get(&new_pos).is_some() {
            return "Yes";
        } else {
            hash_set.insert(new_pos);
        }

        pos = new_pos;
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "RLURU", "Yes"),
            TestCase(20, "URDDLLUUURRRDDDDLLLL", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
