// https://atcoder.jp/contests/joi2021yo1a/tasks/joi2021_yo1a_c

use std::collections::HashSet;

use itertools::Itertools;

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    let mut set_a = HashSet::new();
    let mut set_b = HashSet::new();

    for n in a {
        set_a.insert(n);
    }

    for n in b {
        set_b.insert(n);
    }

    set_a
        .intersection(&set_b)
        .cloned()
        .sorted()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 5, vec![2, 4, 6, 8, 10, 12], vec![15, 12, 9, 6, 3], vec![6, 12]),
            TestCase(3, 5, vec![100, 100, 100], vec![100, 100, 100, 100, 100], vec![100]),
            TestCase(1, 1, vec![2], vec![5], Vec::new()),
            TestCase(10, 10, vec![76, 91, 98, 7, 98, 7, 98, 10, 63, 91], vec![70, 71, 10, 10, 91, 70, 65, 10, 63, 76], vec![10, 63, 76, 91]),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
