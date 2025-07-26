// https://atcoder.jp/contests/abc408/tasks/abc408_c

fn run(n: usize, _m: usize, lr: Vec<(usize, usize)>) -> usize {
    let mut imos: Vec<isize> = vec![0; n];

    for (l, r) in lr {
        imos[l-1] += 1;

        if n != r {
            imos[r] -= 1;
        }
    }

    let mut acc = vec![imos[0]];

    for i in 1..imos.len() {
        acc.push(imos[i] + acc[i-1]);
    }

    acc.into_iter()
        .min()
        .unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn abc408_c() {
        let tests = [
            TestCase(10, 4, vec![(1, 6), (4, 5), (5, 10), (7, 10)], 1),
            TestCase(5, 2, vec![(1, 2), (3, 4)], 0),
            TestCase(5, 10, vec![(2, 5), (1, 5), (1, 2), (2, 4), (2, 2), (5, 5), (2, 4), (1, 2), (2, 2), (2, 3)], 3),
        ];

        for TestCase(n, m, lr, expected) in tests {
            assert_eq!(run(n, m, lr), expected);
        }
    }
}
