// https://atcoder.jp/contests/joig2024-open/tasks/joig2024_b

use itertools::Itertools;

fn run(n: usize, d: usize, a: Vec<usize>) -> &'static str {
    let vec: Vec<usize> = a.into_iter().sorted().collect();

    if (0..n*2-1).into_iter()
        .step_by(2)
        .any(|i| {
            vec[i+1] - vec[i] > d
        }) {
            "No"
        } else {
            "Yes"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 5, vec![2, 7], "Yes"),
            TestCase(3, 0, vec![10, 10, 10, 11, 10, 10], "No"),
            TestCase(6, 4, vec![22, 15, 32, 36, 16, 30, 42, 30, 39, 23, 17, 18], "Yes"),
        ];

        for TestCase(n, d, a, expected) in tests {
            assert_eq!(run(n, d, a), expected);
        }
    }
}
