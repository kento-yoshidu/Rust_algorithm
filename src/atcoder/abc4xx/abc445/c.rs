// https://atcoder.jp/contests/abc445/tasks/abc445_c

fn run(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut ans = vec![0; n];

    for i in (0..n).rev() {
        if a[i] - 1 == i {
            ans[i] = i + 1;
        } else {
            ans[i] = ans[a[i] - 1];
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc445_c() {
        let tests = [
            TestCase(7, vec![2, 4, 7, 5, 5, 6, 7], vec![5, 5, 7, 5, 5, 6, 7]),
            TestCase(5, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            TestCase(15, vec![11, 3, 10, 7, 15, 10, 10, 11, 11, 13, 11, 12, 14, 14, 15], vec![11, 14, 14, 14, 15, 14, 14, 11, 11, 14, 11, 12, 14, 14, 15]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
