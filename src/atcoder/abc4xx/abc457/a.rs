// https://atcoder.jp/contests/abc457/tasks/abc457_a

fn run(n: usize, a: Vec<usize>, x: usize) -> usize {
    a[x-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, usize);

    #[test]
    fn abc457_a() {
        let tests = [
            TestCase(5, vec![1, 2, 3, 4, 5], 3, 3),
            TestCase(10, vec![6, 6, 9, 6, 10, 5, 7, 2, 8, 2], 4, 6),
            TestCase(10, vec![4, 4, 4, 3, 4, 2, 1, 1, 2, 1], 10, 1),
        ];

        for TestCase(n, a, x, expected) in tests {
            assert_eq!(run(n, a, x), expected);
        }
    }
}
