// https://yukicoder.me/problems/no/964

fn run(n: usize) -> String {
    let mut s = String::new();

    for d in 1..=n {
        s.push_str(&d.to_string().repeat(n));
    }

    if n == 10 {
        s.push_str(&"0".repeat(n));
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn yuki_964() {
        let tests = [
            TestCase(1, "1"),
            TestCase(2, "1122"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
