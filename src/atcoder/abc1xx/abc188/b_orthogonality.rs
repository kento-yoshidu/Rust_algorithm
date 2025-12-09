// https://atcoder.jp/contests/abc188/tasks/abc188_b

fn run(_n: usize, a: Vec<isize>, b: Vec<isize>) -> &'static str {
    let total: isize = a.into_iter()
        .zip(b.iter())
        .map(|t| t.0 * t.1 )
        .sum();

    if total == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>, &'static str);

    #[test]
    fn abc188_b() {
        let tests = [
            TestCase(2, vec![-3, 6], vec![4, 2], "Yes"),
            TestCase(2, vec![4, 5], vec![-1, -3], "No"),
            TestCase(3, vec![1, 3, 5], vec![3, -6, 3], "Yes"),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
