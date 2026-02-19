// https://atcoder.jp/contests/abc252/tasks/abc252_b

fn run(_n: usize, _k: usize, a: Vec<usize>, b: Vec<usize>) -> &'static str {
    let max = a.iter().max().unwrap();

    if b.into_iter()
        .any(|i| {
            a[i-1] == *max
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn abc252_b() {
        let tests = [
            TestCase(5, 3, vec![6, 8, 10, 7, 10], vec![2, 3, 4], "Yes"),
            TestCase(5, 2, vec![100, 100, 100, 1, 1], vec![5, 4], "No"),
            TestCase(2, 1, vec![100, 1], vec![2], "No"),
        ];

        for TestCase(n, k, a, b, expected) in tests {
            assert_eq!(run(n, k, a, b), expected);
        }
    }
}
