// https://atcoder.jp/contests/abc473/tasks/abc473_a

fn run(n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .skip(n / 2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc473_a() {
        let tests = [
            TestCase(8, vec![1, 3, 7, 8, 4, 2, 5, 6], 17),
            TestCase(2, vec![1, 100], 100),
            TestCase(10, vec![31, 41, 59, 26, 53, 58, 97, 93, 23, 84], 355),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
