// https://atcoder.jp/contests/abc400/tasks/abc400_b

fn run(n: usize, m: usize) -> String {
    let mut total = 1;

    for i in 1..=m {
        total += n.pow(i as u32);

        if total > 1_000_000_000 {
            return String::from("inf");
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 3, "400"),
            TestCase(1000000, 2, "inf"),
            TestCase(999999999, 1, "1000000000"),
            TestCase(998244353, 99, "inf"),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
