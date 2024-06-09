// https://atcoder.jp/contests/pakencamp-2023-day1/tasks/pakencamp_2023_day1_a

use std::collections::HashMap;

fn run(n: usize, a: Vec<usize>) -> &'static str {
    let mut hash_map = HashMap::new();

    for num in a {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    if hash_map.into_iter()
        .any(|(_, cnt)| {
            cnt <= 10
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec![10, 20, 10, 20, 30, 30], "Yes"),
            TestCase(55, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], "No"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
