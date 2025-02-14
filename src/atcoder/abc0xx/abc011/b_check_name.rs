// https://atcoder.jp/contests/abc011/tasks/abc011_2

fn run(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                (c.to_uppercase()).to_string()
            } else {
                (c.to_lowercase()).to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("taKahAshI", "Takahashi"),
            TestCase("A", "A"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
