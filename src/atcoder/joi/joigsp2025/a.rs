// https://atcoder.jp/contests/joigsp2025/tasks/joigsp2025_a

use std::collections::BTreeSet;

fn run(_n: usize, k: usize, a: Vec<usize>) -> &'static str {
    let mut bt_set = BTreeSet::new();

    for n in a {
        bt_set.insert(n);
    }

    let sorted: Vec<usize> = bt_set.into_iter().collect();

    if k > sorted.len() {
        return "No";
    }

    for i in 0..sorted.len()-k+1 {
        if sorted[i+k-1] - sorted[i] == k-1 {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn joigsp2025_a() {
        let tests = [
            TestCase(5, 2, vec![1, 1, 2, 4, 3], "Yes"),
            TestCase(7, 4, vec![1, 1, 2, 3, 3, 5, 6], "No"),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
