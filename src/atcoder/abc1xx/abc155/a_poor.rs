// https://atcoder.jp/contests/abc155/tasks/abc155_a

use std::collections::HashSet;

fn run(a: usize, b: usize, c: usize) -> &'static str {
    let mut vec = vec![a, b, c];

    vec.sort();
    vec.dedup();

    if vec.len() == 2 {
        "Yes"
    } else {
        "No"
    }
}

fn run2(a: usize, b: usize, c: usize) -> &'static str {
    let hash_set = HashSet::from([a, b, c]);

    if hash_set.len() == 2 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn abc155_a() {
        let tests = [
            TestCase(5, 7, 5, "Yes"),
            TestCase(4, 4, 4, "No"),
            TestCase(4, 9, 6, "No"),
            TestCase(3, 3, 4, "Yes"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
            assert_eq!(run2(a, b, c), expected);
        }
    }
}
