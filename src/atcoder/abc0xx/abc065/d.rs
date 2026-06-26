// https://atcoder.jp/contests/abc065/tasks/arc076_b

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, xy: Vec<(usize, usize)>) -> usize {
    let mut xi: Vec<(usize, usize)> = xy.iter().enumerate().map(|(i, &(x, _))| (x, i)).collect();
    let mut yi: Vec<(usize, usize)> = xy.iter().enumerate().map(|(i, &(_, y))| (y, i)).collect();

    xi.sort();
    yi.sort();

    let mut edges = Vec::new();

    for i in 0..n - 1 {
        let (x1, i1) = xi[i];
        let (x2, i2) = xi[i + 1];
        edges.push((x2 - x1, i1, i2));
    }

    for i in 0..n - 1 {
        let (y1, i1) = yi[i];
        let (y2, i2) = yi[i + 1];
        edges.push((y2 - y1, i1, i2));
    }

    edges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut uf = UnionFind::new(n + 1);
    let mut ans = 0;

    for (c, u, v) in edges {
        if uf.unite(u, v) {
            ans += c;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc065_d() {
        let tests = [
            TestCase(3, vec![(1, 5), (3, 9), (7, 8)], 3),
            TestCase(6, vec![(8, 3), (4, 9), (12, 19), (18, 1), (13, 5), (7, 6)], 8),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
