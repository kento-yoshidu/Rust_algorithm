// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/6/ITP1_6_A

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    a.into_iter()
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn itp1_6_a() {
        let tests = [
            TestCase(5, vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
            TestCase(8, vec![3, 3, 4, 4, 5, 8, 7, 9], vec![9, 7, 8, 5, 4, 4, 3, 3]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
