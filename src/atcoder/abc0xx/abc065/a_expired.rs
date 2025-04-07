// https://atcoder.jp/contests/abc065/tasks/abc065_a

fn run(x: i32, a: i32, b: i32) -> &'static str {
    if a >= b {
        return "delicious";
    }

    let expire = b - a;

    if x >= expire {
        "safe"
    } else {
        "dangerous"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, i32, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, 6, "safe"),
            TestCase(6, 5, 1, "delicious"),
            TestCase(3, 7, 12, "dangerous"),
        ];

        for TestCase(x, a, b, expected) in tests {
            assert_eq!(run(x, a, b), expected);
        }
    }
}
