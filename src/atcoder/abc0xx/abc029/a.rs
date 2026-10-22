// https://atcoder.jp/contests/abc029/tasks/abc029_a

fn run(w: String) -> String {
    w + "s"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc029_a() {
        let tests = [
            TestCase("dog", "dogs"),
            TestCase("chokudai", "chokudais"),
        ];

        for TestCase(w, expected) in tests {
            assert_eq!(run(w.to_string()), expected);
        }
    }
}
