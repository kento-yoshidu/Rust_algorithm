// https://atcoder.jp/contests/abc193/tasks/abc193_c

use std::collections::HashSet;
use num_integer::Roots;

fn run(n: usize) -> usize {
    let mut hash_set: HashSet<usize> = HashSet::new();

    for num in 2..=(n.sqrt()) {
        let mut tmp = num * num;

        loop {
            if tmp > n {
                break;
            }

            hash_set.insert(tmp);
            tmp *= num;
        }
    }

    n - hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, 6),
            TestCase(100000, 99634),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }

    }
}
