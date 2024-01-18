// https://atcoder.jp/contests/abc030/tasks/abc030_b

pub fn run(n: usize, m: usize) -> f64 {
    let minute = m as f64 * 6.0;
    let hour = (n % 12) as f64 * 30.0 + minute / 12.0;

    let angle = (hour - minute).abs();

    if angle > 180.0 {
        360.0 - angle
    } else {
        angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Testcase(usize, usize, f64);

    #[test]
    fn test() {
        let tests = [
            Testcase(15, 0, 90.0),
            Testcase(3, 17, 3.5),
            Testcase(0, 0, 0.0),
            Testcase(6, 0, 180.0),
            Testcase(23, 59, 5.5),
        ];

        for Testcase(n, m, expected) in tests {
            assert_eq!(expected, run(n, m));
        }
    }
}
