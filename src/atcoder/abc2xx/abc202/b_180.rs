// https://atcoder.jp/contests/abc202/tasks/abc202_b

fn run(s: &str) -> String {
    s.chars()
        .rev()
        .map(|c| {
            match c {
                '6' => '9',
                '9' => '6',
                _ => c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc202_b() {
        let tests = [
            TestCase("0601889", "6881090"),
            TestCase("86910", "01698"),
            TestCase("01010", "01010"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
