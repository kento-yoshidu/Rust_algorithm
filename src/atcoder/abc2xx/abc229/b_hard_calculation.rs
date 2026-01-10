// https://atcoder.jp/contests/abc229/tasks/abc229_b

fn calc(a: usize, b: usize) -> bool {
    if a == 0 || b == 0 {
        true
    } else if a%10 + b %10 >= 10 {
        false
    } else {
        calc(a/10, b/10)
    }
}

fn run(a: usize, b: usize) -> &'static str {
    if calc(a, b) == true {
        "Easy"
    } else {
        "Hard"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc229_b() {
        let tests = [
            TestCase(229, 390, "Hard"),
            TestCase(123456789, 9876543210, "Easy"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
