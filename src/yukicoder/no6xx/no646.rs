// https://yukicoder.me/problems/no/646

fn run(n: usize) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();

    for i in (1..=n).rev() {
        let mut vec = Vec::new();

        for _ in 0..i {
            vec.push(n);
        }

        ans.push(vec)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>);

    #[test]
    fn yuki_646() {
        let tests = [
            TestCase(5, vec![vec![5, 5, 5, 5, 5], vec![5, 5, 5, 5], vec![5, 5, 5], vec![5, 5], vec![5]]),
            TestCase(1, vec![vec![1]]),
            TestCase(10, vec![vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10], vec![10, 10, 10, 10, 10, 10, 10, 10, 10], vec![10, 10, 10, 10, 10, 10, 10, 10], vec![10, 10, 10, 10, 10, 10, 10], vec![10, 10, 10, 10, 10, 10], vec![10, 10, 10, 10, 10], vec![10, 10, 10, 10], vec![10, 10, 10], vec![10, 10], vec![10]]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
