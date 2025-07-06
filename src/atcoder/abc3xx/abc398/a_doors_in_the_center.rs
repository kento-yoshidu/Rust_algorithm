// https://atcoder.jp/contests/abc398/tasks/abc398_a

fn run(n: usize) -> String {
    if n % 2 == 1 {
        format!("{}{}{}", "-".repeat(n/2), "=", "-".repeat(n/2))
    } else {
        format!("{}{}{}", "-".repeat(n/2-1), "==", "-".repeat(n/2-1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "-==-"),
            TestCase(7, "---=---"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
