// https://atcoder.jp/contests/abc395/tasks/abc395_a

fn run(_n: usize, a: Vec<usize>) -> &'static str {
    if a.windows(2)
        .any(|arr | arr[0] >= arr[1]) {
            "No"
        } else {
            "Yes"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 5], "Yes"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
