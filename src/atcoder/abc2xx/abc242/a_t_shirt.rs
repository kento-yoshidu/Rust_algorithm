// https://atcoder.jp/contests/abc242/tasks/abc242_a

fn run(a: usize, b: usize, c: usize, x: usize) -> f64 {
    if x <= a {
        return 1.0
    }

    if b < x {
        return 0.0
    }

    c as f64 / (b - a) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, f64);

    #[test]
    fn abc242_a() {
        let tests = [
            TestCase(30, 500, 20, 103, 0.0425531914893617),
            TestCase(50, 500, 100, 1, 1.0),
            TestCase(1, 2, 1, 1000, 0.0),
        ];

        for TestCase(a, b, c, x, expected) in tests {
            assert_eq!(run(a, b, c, x), expected);
        }
    }
}

