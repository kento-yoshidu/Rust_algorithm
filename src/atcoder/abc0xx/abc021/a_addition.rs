// https://atcoder.jp/contests/abc021/tasks/abc021_a

fn run(n: usize) -> Vec<usize> {
    let mut ans = vec![n];

    for i in 0..4 {
        if n & 1 << i != 0 {
            ans.push(1 << i)
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, vec![10, 2, 8]),
            TestCase(9, vec![9, 1, 8]),
            TestCase(8, vec![8, 8]),
            TestCase(7, vec![7, 1, 2, 4]),
            TestCase(6, vec![6, 2, 4]),
            TestCase(5, vec![5, 1, 4]),
            TestCase(4, vec![4, 4]),
            TestCase(3, vec![3, 1, 2]),
            TestCase(2, vec![2, 2]),
            TestCase(1, vec![1, 1]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
