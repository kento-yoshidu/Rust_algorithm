// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_c

pub fn run(n: usize, k: usize, a: Vec<usize>) -> Vec<&'static str> {
    (k..n).map(|i| {
        if a[i] > a[i-k] {
            "Yes"
        } else {
            "No"
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, vec![96, 98, 95, 100, 20], vec!["Yes", "No"]),
            TestCase(3, 2, vec![1001, 869120, 1001], vec!["No"]),
            TestCase(15, 7, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9], vec!["Yes", "Yes", "No", "Yes", "Yes", "No", "Yes", "Yes"]),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
