// https://atcoder.jp/contests/abc057/tasks/abc057_b

pub fn run(_n: usize, _m: usize, a: Vec<(isize, isize)>, c: Vec<(isize, isize)>) -> Vec<isize> {
    a.iter()
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

    #[test]
    fn test() {
        assert_eq!(vec![2, 1], run(2, 2, vec![(2, 0), (0, 0)], vec![(-1, 0), (1, 0)]));
        assert_eq!(vec![3, 1, 2], run(3, 4, vec![(10, 10), (-10, -10), (3, 3)], vec![(1, 2), (2, 3), (3, 5), (3, 5)]));
        assert_eq!(vec![5, 4, 3, 2, 1], run(5, 5, vec![(-100000000, -100000000), (-100000000, 100000000), (100000000, -100000000), (100000000, 100000000), (0, 0)], vec![(0, 0), (100000000, 100000000), (100000000, -100000000), (-100000000, 100000000), (-100000000, -100000000)]));
    }
}
