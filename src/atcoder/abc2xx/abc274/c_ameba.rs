// https://atcoder.jp/contests/abc274/tasks/abc274_c

fn run(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut ans = vec![0; 2*n + 1];

    for (i, x) in a.iter().enumerate() {
        ans[i*2 + 1] = ans[*x-1] + 1;
        ans[i*2 + 2] = ans[*x-1] + 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![1, 2], vec![0, 1, 1, 2, 2]),
            TestCase(4, vec![1, 3, 5, 2], vec![0, 1, 1, 2, 2, 3, 3, 2, 2]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
