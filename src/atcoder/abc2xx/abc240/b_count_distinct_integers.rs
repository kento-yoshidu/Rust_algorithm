// https://atcoder.jp/contests/abc240/tasks/abc240_b

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut vec = a.clone();

    vec.sort();
    vec.dedup();

    vec.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc240_b() {
        let tests = [
            TestCase(6, vec![1, 4, 1, 2, 2, 1], 3),
            TestCase(1, vec![1], 1),
            TestCase(11, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5], 7),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
