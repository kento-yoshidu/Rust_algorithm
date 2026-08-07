// https://atcoder.jp/contests/abc461/tasks/abc461_b

fn run(_n: usize, a: Vec<usize>, b: Vec<usize>) -> &'static str {
    if a.into_iter()
        .enumerate()
        .all(|(i, a)| {
            b[a-1] == i+1
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn abc461_b() {
        let tests = [
            TestCase(3, vec![3, 1, 2], vec![2, 3, 1], "Yes"),
            TestCase(4, vec![1, 2, 3, 4], vec![1, 3, 2, 4], "No"),
            TestCase(5, vec![2, 4, 5, 1, 3], vec![4, 1, 5, 2, 3], "Yes"),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
