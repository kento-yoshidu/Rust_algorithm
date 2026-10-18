// https://atcoder.jp/contests/awc0010/tasks/awc0010_a

fn run(_n: usize, m: usize, a: Vec<usize>) -> &'static str {
    if  m <= a.into_iter().sum() {
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
    fn awc0010_a() {
        let tests = [
            TestCase(3, 10, vec![3, 4, 2], "No"),
            TestCase(5, 100, vec![10, 20, 30, 15, 25], "Yes"),
            TestCase(4, 1000000000, vec![250000000, 250000000, 250000000, 250000000], "Yes"),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
