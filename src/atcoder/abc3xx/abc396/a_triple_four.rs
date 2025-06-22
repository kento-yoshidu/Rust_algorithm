// https://atcoder.jp/contests/abc396/tasks/abc396_a

fn run(_n: usize, a: Vec<usize>) -> &'static str {
    if a.windows(3)
        .any(|arr| {
            arr[0] == arr[1] && arr[1] == arr[2]
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 4, 4, 4, 2], "Yes"),
            TestCase(6, vec![2, 4, 4, 2, 2, 4], "No"),
            TestCase(8, vec![1, 4, 2, 5, 7, 7, 7, 2], "Yes"),
            TestCase(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "No"),
            TestCase(13, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], "Yes"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
