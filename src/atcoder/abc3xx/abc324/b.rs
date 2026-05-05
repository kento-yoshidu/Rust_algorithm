// https://atcoder.jp/contests/abc324/tasks/abc324_b

fn run(n: usize) -> &'static str {
    let mut num = n;

    while num % 2 == 0 {
        num /= 2
    }

    while num % 3 == 0 {
        num /= 3;
    }

    if num == 1 {
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
    fn abc324_b() {
        let tests = [
            TestCase(324, "Yes"),
            TestCase(5, "No"),
            TestCase(32, "Yes"),
            TestCase(37748736, "Yes"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n) ,expected);
        }
    }
}
