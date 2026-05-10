// https://atcoder.jp/contests/abc339/tasks/abc339_e

use library::lib::graph::segment_tree::SegTree;

fn run(_n: usize, d: usize, a: Vec<usize>) -> usize {
    const V: usize = 5 * 100000 + 1;

    let mut st = SegTree::new(V, 0, |l, r| l.max(r));

    for n in a {
        let l = n.saturating_sub(d);
        let r = (n + d + 1).min(V);

        let max = st.prod(l, r);

        st.set(n, max + 1);
    }

    st.all_prod()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc339_e() {
        let tests = [
            TestCase(4, 2, vec![3, 5, 1, 2], 3),
            TestCase(5, 10, vec![10, 20, 100, 110, 120], 3),
            TestCase(11, 7, vec![21, 10, 3, 19, 28, 12, 11, 3, 3, 15, 16], 6),
        ];

        for TestCase(n, d, a, expected) in tests {
            assert_eq!(run(n, d, a), expected);
        }
    }
}
