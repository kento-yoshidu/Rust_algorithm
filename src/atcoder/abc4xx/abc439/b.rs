// https://atcoder.jp/contests/abc439/tasks/abc439_b

use std::collections::HashSet;

fn rec(n: u32, set: &mut HashSet<u32>) -> &'static str {
    if n == 1 {
        "Yes"
    } else if set.get(&n).is_some() {
        "No"
    } else {
        set.insert(n);

        let i: u32 = n.to_string().chars()
            .map(|c| {
                c.to_digit(10).unwrap().pow(2)
            })
            .sum();

        rec(i, set)
    }
}

fn run(n: u32) -> &'static str {
    let mut set = HashSet::new();

    rec(n, &mut set)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u32, &'static str);

    #[test]
    fn abc439_b() {
        let tests = [
            TestCase(2026, "Yes"),
            TestCase(439, "No"),
            TestCase(440, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
