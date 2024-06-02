
use std::collections::HashMap;

pub fn run(n: usize, _q: usize, t: Vec<usize>) -> usize {
    let mut hash_map = HashMap::new();

    for number in t {
        *hash_map.entry(number).or_insert(0) += 1;
    }

    let count = hash_map.iter()
        .filter(|(_, val)| {
            **val % 2 != 0
        })
        .count();

    n - count
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(30, 6, vec![2, 9, 18, 27, 18, 9], 28),
            TestCase(1, 7, vec![1, 1, 1, 1, 1, 1, 1], 0),
            TestCase(9, 20, vec![9, 5, 1, 2, 2, 2, 8, 9, 2, 1, 6, 2, 6, 5, 8, 7, 8, 5, 9, 8], 5),
        ];

        for TestCase(n, q, t, expected) in tests {
            assert_eq!(run(n, q, t), expected);
        }
    }
}
