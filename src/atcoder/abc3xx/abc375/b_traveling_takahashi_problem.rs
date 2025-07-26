// https://atcoder.jp/contests/abc375/tasks/abc375_b

fn run(_n: usize, xy: Vec<(isize, isize)>) -> f64 {
    let mut ans = 0.0;

    let mut pos = (0.0, 0.0);

    for (x, y) in xy {
        ans += ((pos.0 - x as f64).powf(2.0) + (pos.1 - y as f64).powf(2.0)).sqrt();
        pos.0 = x as f64;
        pos.1 = y as f64;
    }

    ans += ((pos.0 - 0.0).powf(2.0) + (pos.1 - 0.0).powf(2.0)).sqrt();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, isize)>, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![(1, 2), (-1, 0)], 6.06449510224597979401),
            TestCase(7, vec![(-14142, 13562), (-17320, 50807), (-22360, 67977), (24494, 89742), (-26457, 51311), (28284, 27124), (31622, 77660)], 384694.57587932075868509383),
            TestCase(5, vec![(-100000, 100000), (100000, -100000), (-100000, 100000), (100000, -100000), (-100000, 100000)], 1414213.5623730952),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
