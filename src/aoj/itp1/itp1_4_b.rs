// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/4/ITP1_4_B

use std::f64::consts::PI;

fn run(r: usize) -> (f64, f64) {
    let r = r as f64;

    (r * r * PI, r * 2.0 * PI)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, (f64, f64));

    #[test]
    fn itp1_4_a() {
        let tests = [
            TestCase(2, (12.566370614359172, 12.566370614359172)),
            TestCase(3, (28.274333882308138, 18.84955592153876)),
        ];

        for TestCase(r, expected) in tests {
            assert_eq!(run(r), expected);
        }
    }
}
