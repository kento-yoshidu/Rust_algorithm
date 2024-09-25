// https://atcoder.jp/contests/agc010/tasks/agc010_a

fn run(_n: usize, a: Vec<usize>) -> &'static str {
    if a.into_iter().sum::<usize>() % 2 == 0 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 3], "YES"),
            TestCase(5, vec![1, 2, 3, 4, 5], "NO")
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
