// https://atcoder.jp/contests/abc442/tasks/abc442_c

use std::collections::HashMap;

fn calc(x: usize) -> usize {
    if x < 3 {
        0
    } else {
        x * (x - 1) * (x - 2) / 6
    }
}

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut map = HashMap::new();

    for (a, b) in ab {
        map.entry(a).or_insert_with(|| Vec::new()).push(b);
        map.entry(b).or_insert_with(|| Vec::new()).push(a);
    }

    (1..=n)
        .map(|i| {
            let len = map.get(&i).map_or(0, |v| v.len());
            let x = (n - 1).saturating_sub(len);

            calc(x)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc442_c() {
        let tests = [
            TestCase(6, 5, vec![ (1, 2), (1, 4), (2, 3), (5, 3), (3, 1)], vec![0, 1, 0, 4, 4, 10]),
            TestCase(7, 3, vec![(1, 2), (3, 4), (5, 6)], vec![10, 10, 10, 10, 10, 10, 20]),
            TestCase(6, 9, vec![(3, 6), (2, 5), (2, 3), (4, 3), (1, 5), (6, 2), (3, 1), (5, 3), (2, 4)], vec![1, 0, 0, 1, 0, 1]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
