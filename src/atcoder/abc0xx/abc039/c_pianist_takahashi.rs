// https://atcoder.jp/contests/abc039/tasks/abc039_c

fn run(s: &str) -> &'static str {
    let str = "WBWBWWBWBWBWWBWBWWBWWBWBWWBWBWBWWBWBWWBW";

    let ans = ["Do", "", "Re", "", "Mi", "Fa", "", "So", "", "La", "", "Si"];

    let p = str.find(s).unwrap();

    ans[p]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("WBWBWWBWBWBWWBWBWWBW", "Do"),
            TestCase("WBWWBWBWBWWBWBWWBWWB", "Re"),
            TestCase("WWBWBWBWWBWBWWBWWBWB", "Mi"),
            TestCase("WBWBWBWWBWBWWBWWBWBW", "Fa"),
            TestCase("WBWBWWBWBWWBWWBWBWWB", "So"),
            TestCase("WBWWBWBWWBWWBWBWWBWB", "La"),
            TestCase("WWBWBWWBWWBWBWWBWBWB", "Si"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
