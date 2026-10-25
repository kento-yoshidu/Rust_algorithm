// https://atcoder.jp/contests/awc0020/tasks/awc0020_a

fn run(n: usize, a: Vec<usize>) -> &'static str {
    if a.into_iter().sum::<usize>() % n == 0 {
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
    fn awc0020_a() {
        let tests = [
            TestCase(3, vec![1, 2, 3], "Yes"),
            TestCase(4, vec![1, 2, 3, 5], "No"),
            TestCase(6, vec![0, 1000000000, 500000000, 200000000, 100000000, 200000000], "No"),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
