// https://atcoder.jp/contests/abc057/tasks/abc057_b

fn run(_n: usize, _m: usize, a: Vec<(isize, isize)>, c: Vec<(isize, isize)>) -> Vec<isize> {
    a.into_iter()
        .map(|(a, b)| {
            c.iter()
                .map(|(c, d)| {
                    (a-c).abs() + (b-d).abs()
                })
                .enumerate()
                .fold((isize::MAX, isize::MAX), |(i_a, a), (i_b, b)| {
                    if b < a {
                        (i_b as isize, b)
                    } else {
                        (i_a, a)
                    }
                })
            }
        )
        .map(|t| {
            t.0 + 1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(isize, isize)>, Vec<(isize, isize)>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, vec![(2, 0), (0, 0)], vec![(-1, 0), (1, 0)], vec![2, 1]),
            TestCase(3, 4, vec![(10, 10), (-10, -10), (3, 3)], vec![(1, 2), (2, 3), (3, 5), (3, 5)], vec![3, 1, 2]),
            TestCase(5, 5, vec![(-100000000, -100000000), (-100000000, 100000000), (100000000, -100000000), (100000000, 100000000), (0, 0)], vec![(0, 0), (100000000, 100000000), (100000000, -100000000), (-100000000, 100000000), (-100000000, -100000000)], vec![5, 4, 3, 2, 1]),
        ];

        for TestCase(n, m, a, c, expected) in tests {
            assert_eq!(run(n, m, a, c), expected);
        }
    }
}
