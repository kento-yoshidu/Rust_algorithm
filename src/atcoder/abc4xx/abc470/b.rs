// https://atcoder.jp/contests/abc470/tasks/abc470_b

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut count = vec![0; n + 1];

    for a in a {
        count[a] += 1;
    }

    n - count.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc470_b() {
        let tests = [
            TestCase(4, vec![3, 1, 2, 1], 2),
            TestCase(5, vec![3, 3, 3, 3, 3], 0),
            TestCase(9, vec![4, 2, 3, 3, 4, 1, 2, 7, 1], 7),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
