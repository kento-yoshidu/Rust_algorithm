// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/2/ITP1_2_C

use itertools::Itertools;

fn run(a: usize, b: usize, c: usize) -> Vec<usize> {
    [a, b, c].into_iter().sorted().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>);

    #[test]
    fn itp1_2_c() {
        let tests = [
            TestCase(3, 8, 1, vec![1, 3, 8]),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
