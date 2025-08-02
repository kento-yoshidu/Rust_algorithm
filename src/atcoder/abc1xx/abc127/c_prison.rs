// https://atcoder.jp/contests/abc127/tasks/abc127_c

fn run(_n: usize, _m: usize, lr: Vec<(usize, usize)>) -> usize {
    let mut l_max = 0;
    let mut r_min = std::usize::MAX;

    for (l, r) in lr {
        l_max = l_max.max(l);
        r_min = r_min.min(r);
    }

    if r_min < l_max {
        0
    } else {
        r_min - l_max + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc127_c() {
        let tests = [
            TestCase(4, 2, vec![(1, 3), (2, 4)], 2),
            TestCase(10, 3, vec![(3, 6), (5, 7), (6, 9)], 1),
            TestCase(100000, 1, vec![(1, 100000)], 100000),
        ];

        for TestCase(n, m, lr, expected) in tests {
            assert_eq!(expected, run(n, m, lr));
        }
    }
}
