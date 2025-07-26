// https://atcoder.jp/contests/abc101/tasks/abc101_b

fn rec(num: usize, sum: usize) -> usize {
    if num == 0 {
        sum
    } else {
        rec(num/10, sum + num % 10)
    }
}

fn run(n: usize) -> &'static str {
    let num = rec(n, 0);

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
    fn abc101_b() {
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
