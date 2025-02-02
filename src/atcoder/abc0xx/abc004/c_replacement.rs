// https://atcoder.jp/contests/abc004/tasks/abc004_3

fn run(n: usize) -> String {
    let mut ans = vec!['1', '2', '3', '4', '5', '6'];

    let n = n % 30;

    for i in 0..n {
        ans.swap(i % 5, i % 5 +1);
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, "213456"),
            TestCase(5, "234561"),
            TestCase(22, "615234"),
            TestCase(10, "345612"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
