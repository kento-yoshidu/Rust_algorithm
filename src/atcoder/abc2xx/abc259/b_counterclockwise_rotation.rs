// https://atcoder.jp/contests/abc259/tasks/abc259_b

use std::f64::consts::PI;

fn run(a: isize, b: isize, d: isize) -> (f64, f64) {
    let rad = d as f64 /180. * PI;

    let x = a as f64 *rad.cos() - b as f64 *rad.sin();
    let y = a as f64 *rad.sin() + b as f64 *rad.cos();

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, (f64, f64));

    #[test]
    fn abc259_b() {
        let tests = [
            TestCase(2, 2, 180, (-2.0000000000000004, -1.9999999999999998)),
            TestCase(5, 0, 120, (-2.49999999999999911182, 4.33012701892219364908)),
            TestCase(0, 0, 11, (0.00000000000000000000, 0.00000000000000000000)),
            TestCase(15, 5, 360, (15.000000000000002, 4.9999999999999964)),
            TestCase(-505, 191, 278, (118.85878514480687, 526.6674369978655)),
        ];

        for TestCase(a, b, d, expected) in tests {
            assert_eq!(run(a, b, d), expected);
        }
    }
}
