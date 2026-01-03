// https://yukicoder.me/problems/no/29

fn run(_n: usize, a: Vec<Vec<usize>>) -> usize {
    let mut ans = 0;

    let mut arr  = vec![0; 11];

    for v in a {
        for n in v {
            arr[n] += 1;
        }
    }

    let mut remain = 0;

    for i in 1..arr.len() {
        ans += arr[i] / 2;
        remain += arr[i] % 2;
    }

    ans + remain / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>, usize);

    #[test]
    fn yuki_029() {
        let tests = [
            TestCase(5, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 1, 2], vec![3, 4, 5]], 6),
            TestCase(3, vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 4),
            TestCase(3, vec![vec![1, 2, 3], vec![5, 4, 1], vec![1, 9, 2]], 3),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
