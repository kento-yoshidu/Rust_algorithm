// https://atcoder.jp/contests/abc390/tasks/abc390_b

fn run(n: usize, a: Vec<usize>) -> &'static str {
    for i in 0..n-2 {
        if a[i]*a[i+2] != a[i+1]*a[i+1] {
            return "No";
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![3, 6, 12, 24, 48], "Yes"),
            TestCase(3, vec![1, 2, 3], "No"),
            TestCase(2, vec![10, 8], "Yes"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
