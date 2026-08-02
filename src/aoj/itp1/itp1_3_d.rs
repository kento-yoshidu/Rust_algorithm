// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/3/ITP1_3_D

fn run(a: usize, b: usize, c: usize) -> usize {
    (a..=b)
        .filter(|num| c % num == 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn itp1_3_d() {
        let tests = [
            TestCase(5, 14, 80, 3),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
