// https://atcoder.jp/contests/abc168/tasks/abc168_c

use std::f64;

fn run(a: usize, b: usize, h: usize, m: usize) -> f64 {
    let a = a as f64;
    let b = b as f64;
    let h = h as f64;
    let m = m as f64;

    let al = (h * 60.0 + m) / 720.0 * (f64::consts::PI * 2.0);
    let beta = m / 60.0 * (f64::consts::PI * 2.0);

    let xa = a * f64::cos(al);
    let ya = a * f64::sin(al);
    let xb = b * f64::cos(beta);
    let yb = b * f64::sin(beta);

    ((xa-xb)*(xa-xb) + (ya-yb)*(ya-yb)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, 9, 0, 5.000000000000001),
            TestCase(3, 4, 10, 40, 4.564257194330056),
        ];

        for TestCase(a, b, h, m, expected) in tests {
            assert_eq!(run(a, b, h, m), expected);
        }
    }
}
