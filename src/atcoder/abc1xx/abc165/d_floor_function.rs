// https://atcoder.jp/contests/abc165/tasks/abc165_d

fn run(a: f64, b: f64, n: f64) -> usize {
    let x = f64::min(b - 1.0, n) as f64;

    ((a*x/b).floor() - a * (x/b).floor()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, f64, f64, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5.0, 7.0, 4.0, 2),
            TestCase(11.0, 10.0, 9.0, 9),
        ];

        for TestCase(a, b, n, expected) in tests {
            assert_eq!(run(a, b, n), expected);
        }
    }
}
