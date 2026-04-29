// https://atcoder.jp/contests/abc040/tasks/abc040_d

use std::collections::BinaryHeap;
use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, aby: Vec<(usize, usize, usize)>, q: usize, vw: Vec<(usize, usize)>) -> Vec<usize> {
    let mut wvi = BinaryHeap::new();

    for (i, (v, w)) in vw.into_iter().enumerate() {
        wvi.push((w, v, i));
    }

    let mut yab = BinaryHeap::new();

    for (a, b, y) in aby {
        yab.push((y, a, b));
    }

    let mut uf = UnionFind::new(n + 1);
    let mut ans = vec![0; q];

    while let Some((w, v, i)) = wvi.pop() {
        while let Some(&(y, _, _)) = yab.peek() {
            if y > w {
                let (_, a, b) = yab.pop().unwrap();
                uf.unite(a, b);
            } else {
                break;
            }
        }

        ans[i] = uf.size(v);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize, usize)>, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc040_d() {
        let tests = [
            TestCase(5, 4, vec![(1, 2, 2000), (2, 3, 2004), (3, 4, 1999), (4, 5, 2001)], 3, vec![(1, 2000), (1, 1999), (3, 1995)], vec![1, 3, 5]),
            TestCase(4, 5, vec![ (1, 2, 2005), (3, 1, 2001), (3, 4, 2002), (1, 4, 2004), (4, 2, 2003)], 5, vec![ (1, 2003), (2, 2003), (1, 2001), (3, 2003), (4, 2004)], vec![3, 3, 4, 1, 1]),
            TestCase(4, 5, vec![(1, 2, 10), (1, 2, 1000), (2, 3, 10000), (2, 3, 100000), (3, 1, 200000)], 4, vec![(1, 0), (2, 10000), (3, 100000), (4, 0)], vec![3, 3, 2, 1]),
        ];

        for TestCase(n, m, aby, q, vw, expected) in tests {
            assert_eq!(run(n, m, aby, q, vw), expected);
        }
    }
}
