// https://atcoder.jp/contests/abc163/tasks/abc163_c

fn run(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut ans = vec![0; n];

    for i in a {
        ans[i-1] += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc163_c() {
        let tests = [
            TestCase(5, vec![1, 1, 2, 2], vec![2, 2, 0, 0, 0]),
            TestCase(10, vec![1, 1, 1, 1, 1, 1, 1, 1, 1], vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            TestCase(7, vec![1, 2, 3, 4, 5, 6], vec![1, 1, 1, 1, 1, 1, 0]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a) ,expected);
        }
    }
}
