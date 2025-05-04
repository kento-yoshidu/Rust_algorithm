// https://atcoder.jp/contests/abc101/tasks/abc101_b

fn calc(num: usize, sum: usize) -> usize {
    if num == 0 {
        sum
    } else {
        calc(num/10, sum + num % 10)
    }
}

fn run(n: usize) -> &'static str {
    let num = calc(n, 0);

    if n % num == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(12, "Yes"),
            TestCase(101, "No"),
            TestCase(999999999, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
