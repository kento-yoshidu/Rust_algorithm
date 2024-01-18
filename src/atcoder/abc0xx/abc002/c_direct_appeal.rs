// https://atcoder.jp/contests/abc002/tasks/abc002_3

pub fn run(xa: isize, ya: isize, xb: isize, yb: isize, xc: isize, yc: isize) -> f64 {
    let a = (xb - xa) as f64;
    let b = (yb - ya) as f64;
    let c = (xc - xa) as f64;
    let d = (yc - ya) as f64;

    (a*d - b*c).abs() / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize, isize, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 0, 3, 0, 2, 5, 5.0),
            TestCase(-1, -2, 3, 4, 5, 6, 2.0),
            TestCase(298, 520, 903, 520, 4, 663, 43257.5),
        ];

        for TestCase(xa, ya, xb, yb, xc, yc, expected) in tests {
            assert_eq!(expected, run(xa, ya, xb, yb, xc, yc));
        }
    }
}
