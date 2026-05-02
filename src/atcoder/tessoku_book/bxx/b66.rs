// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_em

use library::lib::graph::union_find::UnionFind;

pub fn run(n: usize, m: usize, ab: Vec<(usize, usize)>, q: usize, query: Vec<(usize, usize, Option<usize>)>) -> Vec<&'static str> {
    let mut removed = vec![false; m + 1];

    for (q, x, _) in query.iter() {
        if *q == 1 {
            removed[*x - 1] = true;
        }
    }

    let mut uf = UnionFind::new(n + 1);

    for i in 0..m {
        if !removed[i] {
            let (a, b) = ab[i];

            uf.unite(a, b);
        }
    }

    let mut ans = Vec::new();

    for (q, x, y) in query.into_iter().rev() {
        match q {
            1 => {
                let (a, b) = ab[x-1];

                uf.unite(a, b);
            },
            2 => {
                let v = y.unwrap();

                if uf.same(x, v) {
                    ans.push("Yes");
                } else {
                    ans.push("No");
                }
            },
            _ => unreachable!(),
        }
    }

    ans.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize, Vec<(usize, usize, Option<usize>)>, Vec<&'static str>);

    #[test]
    fn tessoku_b66() {
        let tests = [
            TestCase(3, 2, vec![(1, 2), (2, 3)], 4, vec![(2, 2, Some(3)), (1, 2, None), (2, 1, Some(3)), (1, 1, None)], vec!["Yes", "No"]),
            TestCase(12, 7, vec![(8, 11), (1, 7), (10, 12), (1, 4), (4, 8), (5, 9), (3, 5)], 12, vec![(2, 6, Some(8)), (1, 6, None), (2, 10, Some(12)), (1, 1, None), (1, 5, None), (1, 3, None), (2, 3, Some(5)), (1, 7, None), (2, 3, Some(6)), (1, 4, None), (1, 2, None), (2, 9, Some(11))], vec!["No", "Yes", "Yes", "No", "No"]),
            TestCase(4, 3, vec![(1, 2), (2, 3), (3, 4)], 7, vec![ (2, 1, Some(2)), (2, 1, Some(3)), (2, 1, Some(4)), (1, 2, None), (2, 1, Some(2)), (2, 1, Some(3)), (2, 1, Some(4))], vec!["Yes", "Yes", "Yes", "Yes", "No", "No"]),
        ];

        for TestCase(n, m, ab, q, query, expected) in tests {
            assert_eq!(run(n, m, ab, q, query), expected);
        }
    }
}
