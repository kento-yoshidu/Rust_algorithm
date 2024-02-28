// https://atcoder.jp/contests/abc255/tasks/abc255_b

pub fn run(_n: usize, _k: usize, a: Vec<usize>, xy: Vec<(isize, isize)>) -> f64 {
    let mut ans = 0;

    for (x, y) in xy.iter() {
        let mut dis = std::isize::MAX;

        for i in a.iter() {
            dis = dis.min((xy[*i-1].0 - x).pow(2) + (xy[*i-1].1 - y).pow(2));
        }

        ans = ans.max(dis);
    }

    (ans as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<(isize, isize)>, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 2, vec![2, 3], vec![(0, 0), (0, 1), (1, 2), (2, 0)], 2.23606797749979),
            TestCase(2, 1, vec![2], vec![(-100000, -100000), (100000, 100000)], 282842.71247461904),
            TestCase(8, 3, vec![2, 6, 8], vec![(-17683, 17993), (93038, 47074), (58079, -57520), (-41515, -89802), (-72739, 68805), (24324, -73073), (71049, 72103), (47863, 19268)],  130379.28045897477),
        ];

        for TestCase(n, k, a, xy, expected) in tests {
            assert_eq!(run(n, k, a, xy), expected);
        }
    }
}
