// https://atcoder.jp/contests/awc0075/tasks/awc0075_a

fn run(_n: usize, r: Vec<usize>) -> usize {
    r.windows(3)
        .filter(|arr| {
            arr[0] < arr[1] && arr[1] > arr[2] ||
            arr[0] > arr[1] && arr[1] < arr[2]
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn awc0075_a() {
        let tests = [
            TestCase(5, vec![1, 3, 2, 4, 1], 3),
            TestCase(8, vec![1, 2, 3, 4, 5, 6, 7, 8], 0),
            TestCase(10, vec![10, 5, 8, 3, 7, 2, 9, 1, 6, 4], 8),
        ];

        for TestCase(n, r, expected) in tests {
            assert_eq!(run(n, r), expected);
        }
    }
}
