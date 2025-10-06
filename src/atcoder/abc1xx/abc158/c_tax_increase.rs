// https://atcoder.jp/contests/abc158/tasks/abc158_c

fn run(a: f64, b: f64) -> i32 {
    for i  in 10..=10000 {
        let tmp = f64::from(i);

        if (tmp * 0.08).floor() == a && (tmp * 0.1).floor() == b {
            return i;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, f64, i32);

    #[test]
    fn abc158_c() {
        let tests = [
            TestCase(2.0, 2.0, 25),
            TestCase(8.0, 10.0, 100),
            TestCase(19.0, 99.0, -1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
