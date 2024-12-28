// https://atcoder.jp/contests/joi2025yo1b/tasks/joi2025_yo1b_d

fn run(n: usize, a: Vec<usize>) -> Vec<Vec<usize>> {
    (0..n-1)
        .scan(a, |state, _| {
            let next: Vec<usize> = state.windows(2)
                .map(|arr| arr[0] + arr[1])
                .collect();

            *state = next;

            Some(state.clone())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<Vec<usize>>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 3, 5, 7, 9], vec![vec![4, 8, 12, 16], vec![12, 20, 28], vec![32, 48], vec![80]]),
            TestCase(7, vec![1, 2, 3, 4, 3, 2, 1], vec![vec![3, 5, 7, 7, 5, 3], vec![8, 12, 14, 12, 8], vec![20, 26, 26, 20], vec![46, 52, 46], vec![98, 98], vec![196]]),
            TestCase(10, vec![1, 9, 2, 4, 4, 9, 2, 3, 5, 6], vec![vec![10, 11, 6, 8, 13, 11, 5, 8, 11], vec![21, 17, 14, 21, 24, 16, 13, 19], vec![38, 31, 35, 45, 40, 29, 32], vec![69, 66, 80, 85, 69, 61], vec![135, 146, 165, 154, 130], vec![281, 311, 319, 284], vec![592, 630, 603], vec![1222, 1233], vec![2455]]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
