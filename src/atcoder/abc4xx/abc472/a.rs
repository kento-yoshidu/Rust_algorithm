// https://atcoder.jp/contests/abc472/tasks/abc472_a

fn run(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == 'A' {
                c
            } else {
                '.'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc472_a() {
        let tests = [
            TestCase("ATCODER", "A......"),
            TestCase("BANANA", ".A.A.A"),
            TestCase("CORRECT", "......."),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
