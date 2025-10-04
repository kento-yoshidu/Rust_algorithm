// https://atcoder.jp/contests/abc152/tasks/abc152_b

fn run(a: usize, b: usize) -> String {
    a.min(b).to_string().repeat(a.max(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc152_b() {
        let tests = [
            TestCase(4, 3, "3333"),
            TestCase(7, 7, "7777777"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
