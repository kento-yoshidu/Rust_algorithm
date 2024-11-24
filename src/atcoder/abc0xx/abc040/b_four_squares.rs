// https://atcoder.jp/contests/abc040/tasks/abc040_b

fn run(n: isize) -> isize {
    let mut ans = std::isize::MAX;

    for i in 1..n {
        let amari = n % i;

        let tmp = amari + (i - (n / i)).abs();

        ans = ans.min(tmp);
    }

    ans
}

fn run2(n: isize) -> isize {
    (1..n)
        .map(|i| {
            let amari = n % i;

            amari + (i - (n / i)).abs()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(26, 1),
            TestCase(41, 4),
            TestCase(100000, 37),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
            assert_eq!(run2(n), expected);
        }
    }
}
