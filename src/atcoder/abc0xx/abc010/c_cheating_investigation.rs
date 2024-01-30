// https://atcoder.jp/contests/abc010/tasks/abc010_3

pub fn run(txa: isize, tya: isize, txb: isize, tyb: isize, t: usize, v: usize, _n: usize, xy: Vec<(isize, isize)>) -> &'static str {
    for (x, y) in xy {
        let a = (((x - txa).pow(2) + (y - tya).pow(2)) as f64).sqrt();
        let b = (((x - txb).pow(2) + (y - tyb).pow(2)) as f64).sqrt();

        if a + b <= (t * v) as f64 {
            return "YES"
        }
    }

    "NO"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, usize, usize, usize, Vec<(isize, isize)>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 1, 8, 2, 2, 4, 1, vec![(4, 5)], "NO"),
            TestCase(1, 1, 8, 2, 2, 6, 1, vec![(4, 5)], "YES"),
            TestCase(1, 1, 8, 2, 2, 5, 1, vec![(4, 5)], "YES"),
            TestCase(7, 7, 1, 1, 3, 4, 3, vec![(8, 1), (1, 7), (9, 9)], "YES"),
        ];

        for TestCase(txa, tya, txb, tyb, t, v, n, xy, expected) in tests {
            assert_eq!(expected, run(txa, tya, txb, tyb, t, v, n, xy));
        }

    }
}
