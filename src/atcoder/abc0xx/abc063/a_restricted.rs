// https://atcoder.jp/contests/abc063/tasks/abc063_a

fn run(a: usize, b: usize) -> String {
    if a + b >= 10 {
        String::from("error")
    } else {
        (a + b).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 3, "9"),
            TestCase(6, 4, "error"),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
