// https://atcoder.jp/contests/abc183/tasks/abc183_b

fn run(sx: isize, sy: isize, gx: isize, gy: isize) -> f64 {
    ((sx * gy) + (sy * gx)) as f64 / (sy + gy) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, f64);

    #[test]
    fn abc183_b() {
        let tests = [
            TestCase(1, 1, 7, 2, 3.0),
            TestCase(1, 1, 3, 2, 1.6666666666666667),
            TestCase(-9, 99, -999, 9999, -18.705882352941178),
        ];

        for TestCase(sx, sy, gx, gy, expected) in tests {
            assert_eq!(run(sx, sy, gx, gy), expected);
        }
    }
}
