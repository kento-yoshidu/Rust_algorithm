// https://atcoder.jp/contests/abc411/tasks/abc411_b

fn run(n: usize, d: Vec<usize>) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();

    for i in 0..n-1 {
        let mut acc = Vec::new();

        for j in i..n-1 {
            if acc.len() == 0 {
                acc.push(d[j]);
            } else {
                acc.push(d[j] + acc[j-i-1]);
            }
        }

        ans.push(acc);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<Vec<usize>>);

    #[test]
    fn abc411_b() {
        let tests = [
            TestCase(5, vec![5, 10, 2, 3], vec![vec![5, 15, 17, 20], vec![10, 12, 15], vec![2, 5], vec![3]]),
            TestCase(2, vec![100], vec![vec![100]]),
        ];

        for TestCase(n, d, expected) in tests {
            assert_eq!(run(n, d), expected);
        }
    }
}
