// https://atcoder.jp/contests/abc344/tasks/abc344_c

use std::collections::HashSet;

pub fn run(
    _n: usize,
    a: Vec<usize>,
    _m: usize,
    b: Vec<usize>,
    _l: usize,
    c: Vec<usize>,
    _q: usize,
    x: Vec<usize>
) -> Vec<&'static str> {
    let mut hs = HashSet::new();

    for aa in a.iter() {
        for bb in b.iter() {
            for cc in c.iter() {
                hs.insert(aa+bb+cc);
            }
        }
    }

    x.iter()
        .map(|num| {
            if hs.contains(num) {
                "Yes"
            } else {
                "No"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, Vec<usize>, usize, Vec<usize>, usize, Vec<usize>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 3], 2, vec![2, 4], 6, vec![1, 2, 4, 8, 16, 32], 4, vec![1, 5, 10, 50], vec!["No", "Yes", "Yes", "No"]),
        ];

        for TestCase(n, a, m, b, l, c, q, x, expected) in tests {
            assert_eq!(run(n, a, m, b, l, c, q, x), expected);
        }
    }
}
