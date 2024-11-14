// https://atcoder.jp/contests/abc370/tasks/abc370_b

fn run(n: usize, a: Vec<Vec<usize>>) -> usize {
    let mut ans = 1;

    for i in 1..=n {
        if i > ans {
            ans = a[i-1][ans-1];
        } else {
            ans = a[ans-1][i-1];
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![vec![3], vec![2, 4], vec![3, 1, 2], vec![2, 1, 2, 4]], 2),
            TestCase(5, vec![vec![5], vec![5, 5,], vec![5, 5, 5,], vec![5, 5, 5, 5], vec![5, 5, 5, 5, 5]], 5),
            TestCase(6, vec![vec![2], vec![1, 5], vec![1, 6, 3], vec![2, 6, 1, 4], vec![2, 1, 1, 1, 6], vec![5, 6, 1, 2, 2, 5]], 5),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
