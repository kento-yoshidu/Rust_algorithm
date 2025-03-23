// https://atcoder.jp/contests/abc398/tasks/abc398_b

use std::collections::HashMap;

fn run(a: [usize; 7]) -> &'static str {
    let mut hash_map = HashMap::new();

    for num in a {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    let mut three = 0;
    let mut two = 0;

    for count in hash_map.values() {
        if *count >= 3 {
            three += 1;
        } else if *count == 2 {
            two += 1;
        }
    }

    if three >= 2 || three == 1 && two >= 1 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase([usize; 7], &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase([1, 4, 1, 4, 2, 1, 3], "Yes"),
            TestCase([11, 12, 13, 10, 13, 12, 11], "No"),
            TestCase([7, 7, 7, 7, 7, 7, 7], "No"),
            TestCase([13, 13, 1, 1, 7, 4, 13], "Yes"),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
