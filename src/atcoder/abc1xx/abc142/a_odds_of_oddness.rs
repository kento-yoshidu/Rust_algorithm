// https://atcoder.jp/contests/abc142/tasks/abc142_a

fn run(a: usize) -> f64 {
    (a as f64 / 2.0).ceil() / a as f64
}

fn run2(a: usize) -> f64 {
    (1..=a)
        .filter(|num| num % 2 == 1)
        .count() as f64 / a as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64);

    #[test]
    fn abc142_a() {
        let tests = [
            TestCase(4, 0.5),
            TestCase(1, 1.0),
            TestCase(5, 0.6),
        ];

        for TestCase(a, expected) in tests {
            assert_eq!(run(a), expected);
        }
    }
}
