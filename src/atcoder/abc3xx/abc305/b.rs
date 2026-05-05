// https://atcoder.jp/contests/abc305/tasks/abc305_b

use std::collections::HashMap;

fn run(p: &str, q: &str) -> isize {
    let distance = HashMap::from([
        ("A", 0),
        ("B", 3),
        ("C", 4),
        ("D", 8),
        ("E", 9),
        ("F", 14),
        ("G", 23),
    ]);

    ((distance[p] - distance[q]) as isize).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, isize);

    #[test]
    fn abc305_b() {
        let tests = [
            TestCase("A", "C", 4),
            TestCase("G", "B", 20),
            TestCase("C", "F", 10),
        ];

        for TestCase(p, q, expected) in tests {
            assert_eq!(run(p, q), expected);
        }
    }
}
