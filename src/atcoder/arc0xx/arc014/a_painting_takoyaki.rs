// https://atcoder.jp/contests/arc014/tasks/arc014_1

pub fn run(n: usize) -> &'static str {
    if n % 2 == 0 {
        "Blue"
    } else {
        "Red"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "Blue"),
            TestCase(9, "Red"),
            TestCase(13, "Red"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
