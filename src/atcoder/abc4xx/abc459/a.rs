// https://atcoder.jp/contests/abc459/tasks/abc459_a

fn run(x: usize) -> String {
    format!("{}{}", &"HelloWorld"[0..x-1], &"HelloWorld"[x..])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc459_a() {
        let tests = [
            TestCase(5, "HellWorld"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
