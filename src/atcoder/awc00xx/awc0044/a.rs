// https://atcoder.jp/contests/awc0044/tasks/awc0044_a

fn run(n: usize, m: usize, cs: Vec<(usize, usize)>) -> Vec<usize> {
    let mut rest = m;

    let mut ans = vec![0; n];

    for (i, (c, s)) in cs.into_iter().enumerate() {
        if rest >= (c - s) {
            ans[i] = c;
            rest -= c - s;
        } else {
            ans[i] = rest + s;
            rest = 0;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn awc0044_a() {
        let tests = [
            TestCase(3, 8, vec![(10, 3), (8, 5), (6, 2)], vec![10, 6, 2]),
        ];

        for TestCase(n, m, cs, expected) in tests {
            assert_eq!(run(n, m, cs), expected);
        }
    }
}
