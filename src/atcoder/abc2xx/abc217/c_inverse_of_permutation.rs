// https://atcoder.jp/contests/abc217/tasks/abc217_c

fn run(n: usize, p: Vec<usize>) -> Vec<usize> {
    let mut q = vec![0; n];

    for (i, num) in p.into_iter().enumerate() {
        q[num-1] = i+1;
    }

    q
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc217_c() {
        let tests = [
            TestCase(3, vec![2, 3, 1], vec![3, 1, 2]),
            TestCase(3, vec![1, 2, 3], vec![1, 2, 3]),
            TestCase(5, vec![5, 3, 2, 4, 1], vec![5, 3, 2, 4, 1]),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
