// https://atcoder.jp/contests/abc466/tasks/abc466_b

use std::collections::HashMap;

fn run(_n: usize, m: usize, cs: Vec<(usize, usize)>) -> Vec<isize> {
    let mut map = HashMap::new();

    for (c, s) in cs {
        map.entry(c).or_insert_with(Vec::new).push(s);
    }

    (1..=m)
        .map(|i| {
            let Some(v) = map.get(&i) else {
                return -1;
            };

            *v.into_iter().max().unwrap() as isize
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<isize>);

    #[test]
    fn abc466_b() {
        let tests = [
            TestCase(4, 5, vec![(1, 3), (2, 10), (1, 7), (4, 9)], vec![7, 10, -1, 9, -1]),
            TestCase(5, 5, vec![(2, 6), (5, 12), (5, 2), (5, 9), (2, 7)], vec![-1, 7, -1, -1, 12]),
        ];

        for TestCase(n, m, cs, expected) in tests {
            assert_eq!(run(n, m, cs), expected);
        }
    }
}
