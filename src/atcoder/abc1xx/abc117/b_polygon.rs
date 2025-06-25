// https://atcoder.jp/contests/abc117/tasks/abc117_b

fn run(_n: usize, l: Vec<usize>) -> &'static str {
    let total: usize = l.iter().sum();
    let max = l.iter().max().unwrap();

    if total - max > *max {
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
    fn abc117_b() {
        let tests = [
            TestCase(4, vec![3, 8, 5, 1], "Yes"),
            TestCase(4, vec![3, 8, 4, 1], "No"),
            TestCase(10, vec![1, 8, 10, 5, 8, 12, 34, 100, 11, 3], "No"),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, l), expected);
        }
    }
}
