// https://atcoder.jp/contests/abc429/tasks/abc429_b

fn run(_n: usize, m: usize, a: Vec<usize>) -> &'static str {
    let total: usize = a.iter().sum();

    if a.contains(&(total - m)) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn abc429_b() {
        let tests = [
            TestCase(4, 10, vec![3, 2, 3, 4], "Yes"),
            TestCase(5, 16, vec![3, 3, 4, 2, 5], "No"),
            TestCase(6, 16, vec![0, 8, 0, 2, 6, 8], "Yes"),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
