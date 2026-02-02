// https://atcoder.jp/contests/abc246/tasks/abc246_b

fn run(a: isize, b: isize) -> (f64, f64) {
    let d = ((a.pow(2) + b.pow(2)) as f64).sqrt();

    (a as f64 / d, b as f64 / d)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, (f64, f64));

    #[test]
    fn abc246_b() {
        let tests = [
            TestCase(3, 4, (0.600000000000, 0.800000000000)),
            TestCase(1, 0, (1.0, 0.0)),
            TestCase(246, 402, (0.5219648702449755, 0.8529669830832527)),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
