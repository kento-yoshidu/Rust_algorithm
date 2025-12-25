// https://yukicoder.me/problems/no/83

fn run(n: usize) -> usize {
    if n % 2 == 0 {
        let count = n / 2;

        "1".repeat(count).parse::<usize>().unwrap()
    } else {
        let count = (n - 3) / 2;
        format!("7{}", "1".repeat(count)).parse::<usize>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn yuki_083() {
        let tests = [
            TestCase(2, 1),
            TestCase(3, 7),
            TestCase(4, 11),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
