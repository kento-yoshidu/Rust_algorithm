// https://atcoder.jp/contests/abc229/tasks/abc229_e

use library::lib::graph::union_find::UnionFind;

fn run(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut graph = vec![vec![]; n + 1];

    for (a, b) in ab.iter() {
        graph[*a].push(b);
        graph[*b].push(a);
    }

    let mut ans = vec![0; n + 2];

    let mut uf = UnionFind::new(n + 1);
    let mut used = vec![false ; n + 1];
    let mut count = 0;

    for i in (1..=n).rev() {
        count += 1;
        used[i] = true;

        for &to in &graph[i] {
            if used[*to] {
                if uf.unite(i, *to) {
                    count -= 1;
                }
            }
        }

        ans[i] = count;
    }

    (1..=n).map(|i| ans[i+1]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc229_e() {
        let tests = [
            TestCase(6, 7, vec![(1, 2), (1, 4), (1, 5), (2, 4), (2, 3), (3, 5), (3, 6)], vec![1, 2, 3, 2, 1, 0]),
            TestCase(8, 7, vec![(7, 8), (3, 4), (5, 6), (5, 7), (5, 8), (6, 7), (6, 8)], vec![3, 2, 2, 1, 1, 1, 1, 0]),
        ];

        for TestCase(n, m, ab, expected) in tests {
            assert_eq!(run(n, m, ab), expected);
        }
    }
}
