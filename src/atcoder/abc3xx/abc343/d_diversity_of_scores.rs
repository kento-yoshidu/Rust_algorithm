// https://atcoder.jp/contests/abc343/tasks/abc343_d

use std::collections::HashMap;

pub fn run(n: usize, _t: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut cur = vec![0; n];
    let mut hs: HashMap<usize, usize> = HashMap::new();

    let mut ans = Vec::new();

    hs.insert(0, n);

    for (a, b) in ab.iter() {
        let temp = cur[a-1];

        cur[a-1] += b;

        *hs.entry(cur[a-1]).or_insert(0) += 1;

        if let Some(count) = hs.get_mut(&temp) {
            *count -= 1;

            if *hs.get(&temp).unwrap() == 0 {
                hs.remove(&temp);
            }
        }

        ans.push(hs.len());
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, vec![(1, 10), (3, 20), (2, 10), (2, 10)], vec![2, 3, 2, 2]),
            TestCase(1, 3, vec![(1, 3), (1, 4), (1, 3)], vec![1, 1, 1]),
            TestCase(10, 10, vec![(7, 2620), (9, 2620), (8, 3375), (1, 3375), (6, 1395), (5, 1395), (6, 2923), (10, 3375), (9, 5929), (5, 1225)], vec![2, 2, 3, 3, 4, 4, 5, 5, 6, 5]),
        ];

        for TestCase(n, t, ab, expected) in tests {
            assert_eq!(run(n, t, ab), expected);
        }
    }
}
