// https://atcoder.jp/contests/abc196/tasks/abc196_b

fn run(x: &str) -> String {
    if !x.contains(".") {
        return x.to_string()
    }

    let p = x.chars().position(|c| c == '.').unwrap();

    x[0..p].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc196_b() {
        let tests = [
            TestCase("123.456", "123"),
            TestCase("0", "0"),
            TestCase("84939825309432908832902189.9092309409809091329", "84939825309432908832902189"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
