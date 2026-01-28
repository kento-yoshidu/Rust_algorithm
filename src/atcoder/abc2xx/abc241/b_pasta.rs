// https://atcoder.jp/contests/abc241/tasks/abc241_b

use std::collections::HashMap;

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> &'static str {
    // 各長さのパスタが何本あるかのHashMap
    let mut hash_map= HashMap::new();

    for num in a {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    for num in b {
        // numの長さの麺が残り0か、そもそも無ければNoを返す
        if *hash_map.get(&num).unwrap_or(&0) == 0 {
            return "No";
        } else {
            *hash_map.entry(num).or_insert(0) -= 1;
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn abc241_b() {
        let tests = [
            TestCase(3, 2, vec![1, 1, 3], vec![3, 1], "Yes"),
            TestCase(1, 1, vec![1000000000], vec![1], "No"),
            TestCase(5, 2, vec![1, 2, 3, 4, 5], vec![5, 5], "No"),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
