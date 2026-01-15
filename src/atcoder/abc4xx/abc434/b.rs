// https://atcoder.jp/contests/abc434/tasks/abc434_b

use std::collections::BTreeMap;

fn run(_n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<f64> {
    let mut b_map = BTreeMap::new();

    for (a, b) in ab {
        b_map.entry(a)
            .or_insert(Vec::new())
            .push(b as f64);
    }

    b_map.into_iter()
        .map(|(_, a)| a.iter().sum::<f64>() / a.len() as f64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<f64>);

    #[test]
    fn abc434_b() {
        let tests = [
            TestCase(10, 5, vec![(4, 92), (1, 16), (3, 77), (4, 99), (2, 89), (3, 8), (1, 40), (5, 56), (1, 40), (4, 77)], vec![32.00000000000000000000, 89.00000000000000000000, 42.50000000000000000000, 89.33333333333333333333, 56.00000000000000000000]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
