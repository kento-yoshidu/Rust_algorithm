// https://atcoder.jp/contests/abc368/tasks/abc368_a

fn run(n: usize, k: usize, a: Vec<usize>) -> Vec<usize> {
    let mut ans = Vec::new();

    for i in n-k..n {
        ans.push(a[i]);
    }

    for i in 0..n-k {
        ans.push(a[i]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            TestCase(6, 2, vec![1, 2, 1, 2, 1, 2], vec![1, 2, 1, 2, 1, 2]),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
