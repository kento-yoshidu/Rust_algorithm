// https://atcoder.jp/contests/abc371/tasks/abc371_d

fn run(n: usize, x: Vec<isize>, p: Vec<isize>, q: usize, lr: Vec<(isize, isize)>) -> Vec<isize> {
    let mut cum = vec![0];

    for i in 1..=n {
        cum.push(cum[i-1] + p[i-1])
    }

    (0..q)
        .map(|i| {
            let l = x.partition_point(|x| *x < lr[i].0);
            let r = x.partition_point(|x| *x < lr[i].1 + 1);
            cum[r] - cum[l]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>, usize, Vec<(isize, isize)>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![1, 3, 5, 7], vec![1, 2, 3, 4], 4, vec![(1, 1), (2, 6), (0, 10), (2, 2)], vec![1, 5, 10, 0]),
            TestCase(7, vec![-10, -5, -3, -1, 0, 1, 4], vec![2, 5, 6, 5, 2, 1, 7], 8, vec![ (-7, 7), (-1, 5), (-10, -4), (-8, 10), (-5, 0), (-10, 5), (-8, 7), (-8, -3)], vec![26, 15, 7, 26, 18, 28, 26, 11]),
        ];

        for TestCase(n, x, p, q, lr, expected) in tests {
            assert_eq!(run(n, x, p, q, lr), expected);
        }
    }
}
