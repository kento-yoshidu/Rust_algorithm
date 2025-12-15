// https://yukicoder.me/problems/no/656

fn run(s: &str) -> u32 {
    s.chars()
        .map(|c| {
            let n = c.to_digit(10).unwrap();

            match c.to_digit(10).unwrap() {
                0 => 10,
                _ => n
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, u32);

    #[test]
    fn yuki_656() {
        let tests = [
            TestCase("444444444", 36),
            TestCase("111111110", 18),
            TestCase("404328039", 53),
            TestCase("000000000", 90),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
