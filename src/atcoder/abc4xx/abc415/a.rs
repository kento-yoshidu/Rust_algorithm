// https://atcoder.jp/contests/abc415/tasks/abc415_a

fn run(_n: usize, a: Vec<usize>, x: usize) -> &'static str {
    if a.contains(&x) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, &'static str);

    #[test]
    fn abc415_a() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 1, 5], 4, "Yes"),
            TestCase(4, vec![100, 100, 100, 100], 100, "Yes"),
            TestCase(6, vec![2, 3, 5, 7, 11, 13], 1, "No"),
        ];

        for TestCase(n, a, x, expected) in tests {
            assert_eq!(run(n, a, x), expected);
        }
    }
}
