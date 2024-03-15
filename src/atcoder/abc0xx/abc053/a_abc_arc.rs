// https://atcoder.jp/contests/abc053/tasks/abc053_a

pub fn run(x: usize) -> &'static str {
    if x < 1200 {
        "ABC"
    } else {
        "ARC"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1000, "ABC"),
            TestCase(2000, "ARC"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
