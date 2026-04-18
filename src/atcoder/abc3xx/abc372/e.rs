// https://atcoder.jp/contests/abc372/tasks/abc372_e

use std::collections::BTreeSet;
use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _q: usize, qux: Vec<(usize, usize, usize)>) -> Vec<isize> {
    let mut ans = Vec::new();

    let mut sets = vec![BTreeSet::new(); n+1];

    for i in 1..=n {
        sets[i].insert(i);
    }

    let mut uf = UnionFind::new(n + 1);

    for (q, u, x) in qux {
        match q {
            1 => {
                let mut ru = uf.find(u);
                let mut rx = uf.find(x);

                if ru == rx {
                    continue;
                }

                if sets[ru].len() < sets[rx].len() {
                    std::mem::swap(&mut ru, &mut rx);
                }

                uf.unite(ru, rx);

                let small = sets[rx].clone();

                for v in small {
                    sets[ru].insert(v);
                }
            },
            2 => {
                let root = uf.find(u);

                if sets[root].len() < x {
                    ans.push(-1);
                } else {
                    let val = sets[root]
                        .iter()
                        .rev()
                        .nth(x - 1)
                        .unwrap();

                    ans.push(*val as isize);
                }
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, Vec<isize>);

    #[test]
    fn abc372_e() {
        let tests = [
            TestCase(4, 10, vec![ (1, 1, 2), (2, 1, 1), (2, 1, 2), (2, 1, 3), (1, 1, 3), (1, 2, 3), (1, 3, 4), (2, 1, 1), (2, 1, 3), (2, 1, 5)], vec![2, 1, -1, 4, 2, -1]),
        ];

        for TestCase(n, q, qux, expected) in tests {
            assert_eq!(run(n, q, qux), expected);
        }
    }
}
