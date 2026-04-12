// https://atcoder.jp/contests/abc444/tasks/abc444_a

fn run(n: usize) -> &'static str {
    let a = n / 100;
    let b = n % 100 / 10;
    let c = n % 10;

    if a == b && b == c {
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
    fn abc444_a() {
        let tests = [
            TestCase(444, "Yes"),
            TestCase(160, "No"),
            TestCase(999, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
