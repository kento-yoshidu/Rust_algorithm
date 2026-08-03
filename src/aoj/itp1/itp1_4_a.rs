// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/4/ITP1_4_A

fn run(a: usize, b: usize) -> (usize, usize, f64) {
    (a / b, a % b, a as f64 / b as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, (usize, usize, f64));

    #[test]
    fn itp1_4_a() {
        let tests = [
            TestCase(3, 2, (1, 1, 1.5)),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
