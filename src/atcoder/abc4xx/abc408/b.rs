// https://atcoder.jp/contests/abc408/tasks/abc408_b

fn run(_n: usize, a: Vec<usize>) -> (usize, Vec<usize>) {
    let mut a = a.clone();

    a.sort();
    a.dedup();

    (a.len(), a)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, (usize, Vec<usize>));

    #[test]
    fn abc408_b() {
        let tests = [
            TestCase(4, vec![3, 1, 4, 1], (3, vec![1, 3, 4])),
            TestCase(3, vec![7, 7, 7], (1, vec![7])),
            TestCase(8, vec![19, 5, 5, 19, 5, 19, 4, 19], (3, vec![4, 5, 19])),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
