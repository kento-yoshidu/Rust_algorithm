// https://atcoder.jp/contests/abc264/tasks/abc264_e

use std::collections::HashSet;

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    has_power: Vec<bool>,
    city_count: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize, city_n: usize) -> Self {
        let mut has_power = vec![false; n + 1];
        let mut city_count = vec![0; n + 1];

        for i in 1..=n {
            if i <= city_n {
                city_count[i] = 1;
            } else {
                has_power[i] = true;
            }
        }

        Self {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
            has_power,
            city_count,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);

            self.parent[x] = root;
        }

        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize) -> usize {
        let mut x = self.find(x);
        let mut y = self.find(y);

        if x == y {
            return 0;
        }

        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }

        let add: usize = if self.has_power[x] && !self.has_power[y] {
            self.city_count[y]
        } else if !self.has_power[x] && self.has_power[y] {
            self.city_count[x]
        } else {
            0
        };

        self.parent[y] = x;
        self.size[x] += self.size[y];

        self.has_power[x] |= self.has_power[y];
        self.city_count[x] += self.city_count[y];

        add
    }
}

fn run(n: usize, m: usize, e: usize, uv: Vec<(usize, usize)>, q: usize, x: Vec<usize>) -> Vec<usize> {
    let mut removed = vec![false; e + 1];

    for i in x.iter() {
        removed[*i] = true;
    }

    let mut uf = UnionFind::new(n + m, n);

    let mut powered = 0;

    for i in 1..=e {
        if !removed[i] {
            let (u, v) = uv[i - 1];
            powered += uf.unite(u, v);
        }
    }

    let mut ans = vec![0; q];

    for i in (0..q).rev() {
        ans[i] = powered;

        let idx = x[i];
        let (u, v) = uv[idx - 1];

        powered += uf.unite(u, v);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc264_e() {
        let tests = [
            TestCase(5, 5, 10, vec![(2, 3), (4, 10), (5, 10), (6, 9), (2, 9), (4, 8), (1, 7), (3, 6), (8, 10), (1, 8)], 6, vec![3, 5, 8, 10, 2, 7], vec![4, 4, 2, 2, 2, 1]),
        ];

        for TestCase(n, m, e, uv, q, x, expected) in tests {
            assert_eq!(run(n, m, e, uv, q, x), expected);
        }
    }
}
