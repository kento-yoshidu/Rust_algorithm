// https://atcoder.jp/contests/abc461/tasks/abc461_c

use std::collections::HashMap;

fn run(_n: usize, k: usize, m: usize, cv: Vec<(usize, usize)>) -> usize {
    let mut map = HashMap::new();

    for (c, v) in cv {
        map.entry(c).or_insert_with(Vec::new).push(v);
    }

    let mut top = Vec::new();
    let mut tails = Vec::new();

    for values in map.values_mut() {
        values.sort_unstable_by(|a, b| b.cmp(a));

        top.push(values[0]);

        for &value in values.iter().skip(1) {
            tails.push(value);
        }
    }

    top.sort_unstable_by(|a, b| b.cmp(a));

     for &v in top.iter().skip(m) {
        tails.push(v);
    }

    tails.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = 0;

    for v in top.into_iter().take(m) {
        ans += v;
    }

    for v in tails.into_iter().take(k - m) {
        ans += v;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc461_c() {
        let tests = [
            TestCase(5, 3, 2, vec![(1, 30), (1, 40), (1, 50), (2, 10), (3, 20)], 110),
            TestCase(5, 3, 3, vec![(1, 30), (1, 40), (1, 50), (2, 10), (3, 20)], 80),
            TestCase(5, 5, 1, vec![(4, 1000000000), (5, 1000000000), (4, 1000000000), (5, 1000000000), (4, 1000000000)], 5000000000),
        ];

        for TestCase(n, k, m, cv, expected) in tests {
            assert_eq!(run(n, k, m, cv), expected);
        }
    }
}
